import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ChefHat } from "lucide-react";

export interface LoginCardProps {
  onLogin?: () => void;
}

export function LoginCard({ onLogin }: LoginCardProps) {
  return (
    <div className="min-h-screen flex items-center justify-center p-4 bg-background">
      <Card className="w-full max-w-md">
        <CardHeader className="text-center space-y-4">
          <div className="mx-auto rounded-full bg-primary/10 p-4 w-fit">
            <ChefHat className="h-12 w-12 text-primary" />
          </div>
          <CardTitle className="font-serif text-3xl">Trough</CardTitle>
          <CardDescription className="text-base">
            Your personal recipe collection. Create, organize, and share your favorite recipes.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <Button
            className="w-full"
            size="lg"
            onClick={onLogin}
            data-testid="button-login"
          >
            Sign in to get started
          </Button>
          <p className="text-center text-sm text-muted-foreground">
            Sign in with your Replit account to access your recipes from anywhere.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
