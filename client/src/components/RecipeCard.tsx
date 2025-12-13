import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Globe, Lock } from "lucide-react";

export interface RecipeCardProps {
  id: string;
  title: string;
  authorName: string;
  authorInitials: string;
  isPublic: boolean;
  tags: string[];
  preview: string;
  onClick?: () => void;
}

export function RecipeCard({
  title,
  authorName,
  authorInitials,
  isPublic,
  tags,
  preview,
  onClick,
}: RecipeCardProps) {
  return (
    <Card
      className="cursor-pointer hover-elevate active-elevate-2 transition-shadow duration-200"
      onClick={onClick}
      data-testid={`card-recipe-${title.toLowerCase().replace(/\s+/g, "-")}`}
    >
      <CardHeader className="flex flex-row items-start justify-between gap-2 pb-2">
        <div className="flex-1 min-w-0">
          <h3 className="font-serif text-xl font-semibold truncate">{title}</h3>
          <div className="flex items-center gap-2 mt-1">
            <Avatar className="h-6 w-6">
              <AvatarFallback className="text-xs">{authorInitials}</AvatarFallback>
            </Avatar>
            <span className="text-sm text-muted-foreground">{authorName}</span>
          </div>
        </div>
        <Badge
          variant={isPublic ? "secondary" : "outline"}
          className="shrink-0"
        >
          {isPublic ? (
            <Globe className="h-3 w-3 mr-1" />
          ) : (
            <Lock className="h-3 w-3 mr-1" />
          )}
          {isPublic ? "Public" : "Private"}
        </Badge>
      </CardHeader>
      <CardContent className="space-y-4">
        <p className="text-sm text-muted-foreground font-mono line-clamp-3 leading-relaxed">
          {preview}
        </p>
        {tags.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {tags.slice(0, 4).map((chapter) => (
              <Badge key={chapter} variant="outline" className="rounded-full text-xs">
                {chapter}
              </Badge>
            ))}
            {tags.length > 4 && (
              <Badge variant="outline" className="rounded-full text-xs">
                +{tags.length - 4} more
              </Badge>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
