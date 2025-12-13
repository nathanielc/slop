import type { Express } from "express";
import { createServer, type Server } from "http";
import { storage } from "./storage";
import { setupAuth, isAuthenticated } from "./replitAuth";
import { insertRecipeSchema } from "@shared/schema";

export async function registerRoutes(
  httpServer: Server,
  app: Express
): Promise<Server> {
  // Auth middleware
  await setupAuth(app);

  // Auth routes
  app.get("/api/auth/user", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const user = await storage.getUser(userId);
      res.json(user);
    } catch (error) {
      console.error("Error fetching user:", error);
      res.status(500).json({ message: "Failed to fetch user" });
    }
  });

  // User stats
  app.get("/api/stats", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const stats = await storage.getUserStats(userId);
      res.json(stats);
    } catch (error) {
      console.error("Error fetching stats:", error);
      res.status(500).json({ message: "Failed to fetch stats" });
    }
  });

  // Recipes routes
  app.get("/api/recipes", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const recipes = await storage.getRecipesByUser(userId);
      res.json(recipes);
    } catch (error) {
      console.error("Error fetching recipes:", error);
      res.status(500).json({ message: "Failed to fetch recipes" });
    }
  });

  app.get("/api/recipes/recent", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const limit = parseInt(req.query.limit as string) || 5;
      const recipes = await storage.getRecentRecipes(userId, limit);
      res.json(recipes);
    } catch (error) {
      console.error("Error fetching recent recipes:", error);
      res.status(500).json({ message: "Failed to fetch recent recipes" });
    }
  });

  app.get("/api/recipes/public", async (req, res) => {
    try {
      const recipes = await storage.getPublicRecipes();
      res.json(recipes);
    } catch (error) {
      console.error("Error fetching public recipes:", error);
      res.status(500).json({ message: "Failed to fetch public recipes" });
    }
  });

  app.get("/api/recipes/:id", async (req: any, res) => {
    try {
      const result = await storage.getRecipeWithChapters(req.params.id);
      if (!result) {
        return res.status(404).json({ message: "Recipe not found" });
      }
      
      // Check access: recipe must be public or owned by user
      const userId = req.user?.claims?.sub;
      if (!result.recipe.isPublic && result.recipe.userId !== userId) {
        return res.status(403).json({ message: "Access denied" });
      }
      
      res.json(result);
    } catch (error) {
      console.error("Error fetching recipe:", error);
      res.status(500).json({ message: "Failed to fetch recipe" });
    }
  });

  app.post("/api/recipes", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const { chapters: chapterNames, ...recipeData } = req.body;
      
      const parsed = insertRecipeSchema.safeParse({ ...recipeData, userId });
      if (!parsed.success) {
        return res.status(400).json({ message: "Invalid recipe data", errors: parsed.error.errors });
      }

      const recipe = await storage.createRecipe(parsed.data);
      
      if (chapterNames && chapterNames.length > 0) {
        await storage.setRecipeChapters(recipe.id, chapterNames, userId);
      }

      res.status(201).json(recipe);
    } catch (error) {
      console.error("Error creating recipe:", error);
      res.status(500).json({ message: "Failed to create recipe" });
    }
  });

  app.patch("/api/recipes/:id", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const { chapters: chapterNames, ...recipeData } = req.body;

      const recipe = await storage.updateRecipe(req.params.id, userId, recipeData);
      if (!recipe) {
        return res.status(404).json({ message: "Recipe not found" });
      }

      if (chapterNames !== undefined) {
        await storage.setRecipeChapters(recipe.id, chapterNames, userId);
      }

      res.json(recipe);
    } catch (error) {
      console.error("Error updating recipe:", error);
      res.status(500).json({ message: "Failed to update recipe" });
    }
  });

  app.delete("/api/recipes/:id", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const deleted = await storage.deleteRecipe(req.params.id, userId);
      if (!deleted) {
        return res.status(404).json({ message: "Recipe not found" });
      }
      res.status(204).send();
    } catch (error) {
      console.error("Error deleting recipe:", error);
      res.status(500).json({ message: "Failed to delete recipe" });
    }
  });

  // Chapters routes
  app.get("/api/chapters", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const chapters = await storage.getChaptersByUser(userId);
      res.json(chapters);
    } catch (error) {
      console.error("Error fetching chapters:", error);
      res.status(500).json({ message: "Failed to fetch chapters" });
    }
  });

  app.get("/api/chapters/:name/recipes", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const chapterName = decodeURIComponent(req.params.name);
      const recipes = await storage.getRecipesByChapter(userId, chapterName);
      res.json(recipes);
    } catch (error) {
      console.error("Error fetching chapter recipes:", error);
      res.status(500).json({ message: "Failed to fetch chapter recipes" });
    }
  });

  app.delete("/api/chapters/:name", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const chapterName = decodeURIComponent(req.params.name);
      const deleted = await storage.deleteChapter(userId, chapterName);
      if (!deleted) {
        return res.status(404).json({ message: "Chapter not found" });
      }
      res.status(204).send();
    } catch (error) {
      console.error("Error deleting chapter:", error);
      res.status(500).json({ message: "Failed to delete chapter" });
    }
  });

  app.patch("/api/chapters/:name", isAuthenticated, async (req: any, res) => {
    try {
      const userId = req.user.claims.sub;
      const oldName = decodeURIComponent(req.params.name);
      const { name: newName } = req.body;

      if (!newName || typeof newName !== "string") {
        return res.status(400).json({ message: "New chapter name required" });
      }

      const renamed = await storage.renameChapter(userId, oldName, newName);
      if (!renamed) {
        return res.status(404).json({ message: "Chapter not found" });
      }
      res.json({ name: newName });
    } catch (error) {
      console.error("Error renaming chapter:", error);
      res.status(500).json({ message: "Failed to rename chapter" });
    }
  });

  return httpServer;
}
