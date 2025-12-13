import { Switch, Route, useLocation } from "wouter";
import { queryClient } from "./lib/queryClient";
import { QueryClientProvider, useQuery } from "@tanstack/react-query";
import { Toaster } from "@/components/ui/toaster";
import { TooltipProvider } from "@/components/ui/tooltip";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/AppSidebar";
import { ThemeToggle } from "@/components/ThemeToggle";
import { UserMenu } from "@/components/UserMenu";
import { LoginCard } from "@/components/LoginCard";
import { Dashboard } from "@/pages/Dashboard";
import { MyRecipes } from "@/pages/MyRecipes";
import { PublicRecipes } from "@/pages/PublicRecipes";
import { RecipeBook } from "@/pages/RecipeBook";
import { RecipeDetail } from "@/pages/RecipeDetail";
import { NewRecipe } from "@/pages/NewRecipe";
import NotFound from "@/pages/not-found";
import { useAuth } from "@/hooks/useAuth";
import type { User } from "@shared/schema";

function getInitials(user: User): string {
  if (user.firstName && user.lastName) {
    return `${user.firstName[0]}${user.lastName[0]}`.toUpperCase();
  }
  if (user.firstName) {
    return user.firstName.substring(0, 2).toUpperCase();
  }
  if (user.email) {
    return user.email.substring(0, 2).toUpperCase();
  }
  return "U";
}

function getDisplayName(user: User): string {
  if (user.firstName && user.lastName) {
    return `${user.firstName} ${user.lastName}`;
  }
  if (user.firstName) {
    return user.firstName;
  }
  if (user.email) {
    return user.email.split("@")[0];
  }
  return "User";
}

function AppContent() {
  const [, setLocation] = useLocation();
  const { user, isLoading, isAuthenticated } = useAuth();

  const { data: chapters = [] } = useQuery<{ name: string; count: number }[]>({
    queryKey: ["/api/chapters"],
    enabled: isAuthenticated,
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="animate-pulse text-muted-foreground">Loading...</div>
      </div>
    );
  }

  if (!isAuthenticated || !user) {
    return <LoginCard onLogin={() => window.location.href = "/api/login"} />;
  }

  const displayName = getDisplayName(user);
  const initials = getInitials(user);

  const sidebarStyle = {
    "--sidebar-width": "16rem",
    "--sidebar-width-icon": "3rem",
  };

  return (
    <SidebarProvider style={sidebarStyle as React.CSSProperties}>
      <div className="flex h-screen w-full">
        <AppSidebar
          username={displayName}
          initials={initials}
          profileImageUrl={user.profileImageUrl || undefined}
          tags={chapters}
          onNewRecipe={() => setLocation("/new")}
        />
        <div className="flex flex-col flex-1 min-w-0">
          <header className="flex items-center justify-between gap-4 p-4 border-b sticky top-0 z-50 bg-background">
            <SidebarTrigger data-testid="button-sidebar-toggle" />
            <div className="flex items-center gap-2">
              <ThemeToggle />
              <UserMenu
                username={displayName}
                initials={initials}
                profileImageUrl={user.profileImageUrl || undefined}
                onProfile={() => console.log("Profile")}
                onSettings={() => setLocation("/settings")}
                onLogout={() => window.location.href = "/api/logout"}
              />
            </div>
          </header>
          <main className="flex-1 overflow-auto p-6">
            <Switch>
              <Route path="/">
                <Dashboard
                  username={displayName}
                  onRecipeClick={(id) => setLocation(`/recipe/${id}`)}
                />
              </Route>
              <Route path="/recipes">
                <MyRecipes
                  username={displayName}
                  onRecipeClick={(id) => setLocation(`/recipe/${id}`)}
                  onNewRecipe={() => setLocation("/new")}
                />
              </Route>
              <Route path="/public">
                <PublicRecipes
                  onRecipeClick={(id) => setLocation(`/recipe/${id}`)}
                />
              </Route>
              <Route path="/chapter/:tagName">
                {(params) => (
                  <RecipeBook
                    tagName={decodeURIComponent(params.tagName)}
                    username={displayName}
                    onRecipeClick={(id) => setLocation(`/recipe/${id}`)}
                    onEditChapter={() => console.log("Edit chapter")}
                    onDeleteChapter={() => console.log("Delete chapter")}
                    onBrowseRecipes={() => setLocation("/recipes")}
                  />
                )}
              </Route>
              <Route path="/recipe/:id">
                {(params) => (
                  <RecipeDetail
                    recipeId={params.id}
                    username={displayName}
                    onBack={() => window.history.back()}
                  />
                )}
              </Route>
              <Route path="/new">
                <NewRecipe
                  onBack={() => window.history.back()}
                />
              </Route>
              <Route component={NotFound} />
            </Switch>
          </main>
        </div>
      </div>
    </SidebarProvider>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <Toaster />
        <AppContent />
      </TooltipProvider>
    </QueryClientProvider>
  );
}

export default App;
