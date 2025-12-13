import { Link, useLocation } from "wouter";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarHeader,
  SidebarFooter,
} from "@/components/ui/sidebar";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Home, ChefHat, Globe, BookOpen, Plus, BookMarked, Settings } from "lucide-react";

interface NavItem {
  title: string;
  url: string;
  icon: typeof Home;
  badge?: number;
}

interface TagItem {
  name: string;
  count: number;
}

export interface AppSidebarProps {
  username: string;
  initials: string;
  profileImageUrl?: string;
  tags: TagItem[];
  onNewRecipe?: () => void;
}

const mainNavItems: NavItem[] = [
  { title: "Dashboard", url: "/", icon: Home },
  { title: "My Recipes", url: "/recipes", icon: ChefHat },
  { title: "Public Recipes", url: "/public", icon: Globe },
];

export function AppSidebar({ username, initials, profileImageUrl, tags, onNewRecipe }: AppSidebarProps) {
  const [location] = useLocation();

  return (
    <Sidebar>
      <SidebarHeader className="p-4">
        <div className="flex items-center gap-3">
          <Avatar className="h-10 w-10">
            {profileImageUrl && <AvatarImage src={profileImageUrl} alt={username} className="object-cover" />}
            <AvatarFallback>{initials}</AvatarFallback>
          </Avatar>
          <div className="flex-1 min-w-0">
            <p className="font-medium truncate">{username}</p>
            <p className="text-sm text-muted-foreground">Recipe Creator</p>
          </div>
        </div>
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={onNewRecipe}
                  className="bg-primary text-primary-foreground"
                  data-testid="button-new-recipe"
                >
                  <Plus className="h-4 w-4" />
                  <span>New Recipe</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Navigation</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {mainNavItems.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton
                    asChild
                    isActive={location === item.url}
                  >
                    <Link href={item.url} data-testid={`nav-${item.title.toLowerCase().replace(/\s+/g, "-")}`}>
                      <item.icon className="h-4 w-4" />
                      <span>{item.title}</span>
                      {item.badge !== undefined && (
                        <Badge variant="secondary" className="ml-auto">
                          {item.badge}
                        </Badge>
                      )}
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>
            <div className="flex items-center gap-2">
              <BookOpen className="h-4 w-4" />
              <span>My Chapters</span>
            </div>
          </SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {tags.map((tag) => (
                <SidebarMenuItem key={tag.name}>
                  <SidebarMenuButton
                    asChild
                    isActive={location === `/chapter/${encodeURIComponent(tag.name)}`}
                  >
                    <Link
                      href={`/chapter/${encodeURIComponent(tag.name)}`}
                      data-testid={`nav-chapter-${tag.name.toLowerCase().replace(/\s+/g, "-")}`}
                    >
                      <BookMarked className="h-4 w-4" />
                      <span>{tag.name}</span>
                      <Badge variant="outline" className="ml-auto text-xs">
                        {tag.count}
                      </Badge>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
              {tags.length === 0 && (
                <p className="px-4 py-2 text-sm text-muted-foreground">
                  No chapters yet. Assign recipes to chapters to organize your book.
                </p>
              )}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter className="p-2">
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton asChild>
              <Link href="/settings" data-testid="nav-settings">
                <Settings className="h-4 w-4" />
                <span>Settings</span>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  );
}
