import {
  users,
  recipes,
  chapters,
  recipeChapters,
  type User,
  type UpsertUser,
  type Recipe,
  type InsertRecipe,
  type Chapter,
  type InsertChapter,
} from "@shared/schema";
import { db } from "./db";
import { eq, and, desc, sql } from "drizzle-orm";

export interface IStorage {
  // User operations (required for Replit Auth)
  getUser(id: string): Promise<User | undefined>;
  upsertUser(user: UpsertUser): Promise<User>;

  // Recipe operations
  getRecipe(id: string): Promise<Recipe | undefined>;
  getRecipeWithChapters(id: string): Promise<{ recipe: Recipe; chapters: Chapter[] } | undefined>;
  getRecipesByUser(userId: string): Promise<Recipe[]>;
  getPublicRecipes(): Promise<(Recipe & { user: User })[]>;
  getRecipesByChapter(userId: string, chapterName: string): Promise<Recipe[]>;
  createRecipe(recipe: InsertRecipe): Promise<Recipe>;
  updateRecipe(id: string, userId: string, recipe: Partial<InsertRecipe>): Promise<Recipe | undefined>;
  deleteRecipe(id: string, userId: string): Promise<boolean>;
  setRecipeChapters(recipeId: string, chapterNames: string[], userId: string): Promise<void>;

  // Chapter operations
  getChaptersByUser(userId: string): Promise<{ name: string; count: number }[]>;
  getOrCreateChapter(userId: string, name: string): Promise<Chapter>;
  deleteChapter(userId: string, name: string): Promise<boolean>;
  renameChapter(userId: string, oldName: string, newName: string): Promise<boolean>;

  // Stats
  getUserStats(userId: string): Promise<{ totalRecipes: number; publicRecipes: number }>;
  getRecentRecipes(userId: string, limit?: number): Promise<Recipe[]>;
}

export class DatabaseStorage implements IStorage {
  // User operations (required for Replit Auth)
  async getUser(id: string): Promise<User | undefined> {
    const [user] = await db.select().from(users).where(eq(users.id, id));
    return user;
  }

  async upsertUser(userData: UpsertUser): Promise<User> {
    const [user] = await db
      .insert(users)
      .values(userData)
      .onConflictDoUpdate({
        target: users.id,
        set: {
          ...userData,
          updatedAt: new Date(),
        },
      })
      .returning();
    return user;
  }

  // Recipe operations
  async getRecipe(id: string): Promise<Recipe | undefined> {
    const [recipe] = await db.select().from(recipes).where(eq(recipes.id, id));
    return recipe;
  }

  async getRecipeWithChapters(id: string): Promise<{ recipe: Recipe; chapters: Chapter[] } | undefined> {
    const [recipe] = await db.select().from(recipes).where(eq(recipes.id, id));
    if (!recipe) return undefined;

    const recipeChapterRows = await db
      .select({ chapter: chapters })
      .from(recipeChapters)
      .innerJoin(chapters, eq(recipeChapters.chapterId, chapters.id))
      .where(eq(recipeChapters.recipeId, id));

    return {
      recipe,
      chapters: recipeChapterRows.map((r) => r.chapter),
    };
  }

  async getRecipesByUser(userId: string): Promise<Recipe[]> {
    return db
      .select()
      .from(recipes)
      .where(eq(recipes.userId, userId))
      .orderBy(desc(recipes.updatedAt));
  }

  async getPublicRecipes(): Promise<(Recipe & { user: User })[]> {
    const results = await db
      .select({
        recipe: recipes,
        user: users,
      })
      .from(recipes)
      .innerJoin(users, eq(recipes.userId, users.id))
      .where(eq(recipes.isPublic, true))
      .orderBy(desc(recipes.updatedAt));

    return results.map((r) => ({ ...r.recipe, user: r.user }));
  }

  async getRecipesByChapter(userId: string, chapterName: string): Promise<Recipe[]> {
    const results = await db
      .select({ recipe: recipes })
      .from(recipes)
      .innerJoin(recipeChapters, eq(recipes.id, recipeChapters.recipeId))
      .innerJoin(chapters, eq(recipeChapters.chapterId, chapters.id))
      .where(and(eq(chapters.userId, userId), eq(chapters.name, chapterName)))
      .orderBy(desc(recipes.updatedAt));

    return results.map((r) => r.recipe);
  }

  async createRecipe(recipe: InsertRecipe): Promise<Recipe> {
    const [newRecipe] = await db.insert(recipes).values(recipe).returning();
    return newRecipe;
  }

  async updateRecipe(
    id: string,
    userId: string,
    recipe: Partial<InsertRecipe>
  ): Promise<Recipe | undefined> {
    const [updated] = await db
      .update(recipes)
      .set({ ...recipe, updatedAt: new Date() })
      .where(and(eq(recipes.id, id), eq(recipes.userId, userId)))
      .returning();
    return updated;
  }

  async deleteRecipe(id: string, userId: string): Promise<boolean> {
    const result = await db
      .delete(recipes)
      .where(and(eq(recipes.id, id), eq(recipes.userId, userId)))
      .returning();
    return result.length > 0;
  }

  async setRecipeChapters(recipeId: string, chapterNames: string[], userId: string): Promise<void> {
    // Delete existing chapter associations
    await db.delete(recipeChapters).where(eq(recipeChapters.recipeId, recipeId));

    if (chapterNames.length === 0) return;

    // Get or create chapters and associate them
    for (const name of chapterNames) {
      const chapter = await this.getOrCreateChapter(userId, name);
      await db.insert(recipeChapters).values({
        recipeId,
        chapterId: chapter.id,
      });
    }
  }

  // Chapter operations
  async getChaptersByUser(userId: string): Promise<{ name: string; count: number }[]> {
    const results = await db
      .select({
        name: chapters.name,
        count: sql<number>`count(${recipeChapters.id})::int`,
      })
      .from(chapters)
      .leftJoin(recipeChapters, eq(chapters.id, recipeChapters.chapterId))
      .where(eq(chapters.userId, userId))
      .groupBy(chapters.id, chapters.name)
      .orderBy(chapters.name);

    return results;
  }

  async getOrCreateChapter(userId: string, name: string): Promise<Chapter> {
    const [existing] = await db
      .select()
      .from(chapters)
      .where(and(eq(chapters.userId, userId), eq(chapters.name, name)));

    if (existing) return existing;

    const [newChapter] = await db
      .insert(chapters)
      .values({ userId, name })
      .returning();
    return newChapter;
  }

  async deleteChapter(userId: string, name: string): Promise<boolean> {
    const result = await db
      .delete(chapters)
      .where(and(eq(chapters.userId, userId), eq(chapters.name, name)))
      .returning();
    return result.length > 0;
  }

  async renameChapter(userId: string, oldName: string, newName: string): Promise<boolean> {
    const result = await db
      .update(chapters)
      .set({ name: newName })
      .where(and(eq(chapters.userId, userId), eq(chapters.name, oldName)))
      .returning();
    return result.length > 0;
  }

  // Stats
  async getUserStats(userId: string): Promise<{ totalRecipes: number; publicRecipes: number }> {
    const [stats] = await db
      .select({
        totalRecipes: sql<number>`count(*)::int`,
        publicRecipes: sql<number>`count(*) filter (where ${recipes.isPublic} = true)::int`,
      })
      .from(recipes)
      .where(eq(recipes.userId, userId));

    return stats || { totalRecipes: 0, publicRecipes: 0 };
  }

  async getRecentRecipes(userId: string, limit: number = 5): Promise<Recipe[]> {
    return db
      .select()
      .from(recipes)
      .where(eq(recipes.userId, userId))
      .orderBy(desc(recipes.updatedAt))
      .limit(limit);
  }
}

export const storage = new DatabaseStorage();
