# Trough - Personal Recipe Management Platform

## Overview

Trough is a full-stack web application for creating, organizing, and sharing recipes. Users can build their personal recipe collections, organize them into chapters (like cookbook sections), and optionally share recipes publicly. The app features a clean, content-first design inspired by Notion and NYT Cooking, with support for generating print-ready SVG visualizations of recipes.

## User Preferences

Preferred communication style: Simple, everyday language.

## System Architecture

### Frontend Architecture
- **Framework**: React 18 with TypeScript
- **Routing**: Wouter (lightweight React router)
- **State Management**: TanStack React Query for server state caching and synchronization
- **UI Components**: shadcn/ui component library built on Radix UI primitives
- **Styling**: Tailwind CSS with custom CSS variables for theming (light/dark mode support)
- **Build Tool**: Vite for development and production builds

The frontend follows a page-based structure with reusable components. Pages include Dashboard, MyRecipes, PublicRecipes, RecipeBook (chapter view), RecipeDetail, and NewRecipe. Components are organized into UI primitives (`components/ui/`) and application-specific components.

### Backend Architecture
- **Framework**: Express.js with TypeScript
- **API Design**: RESTful JSON API with `/api` prefix
- **Authentication**: Replit OpenID Connect (OIDC) integration with Passport.js
- **Session Management**: PostgreSQL-backed sessions via connect-pg-simple

Routes are defined in `server/routes.ts` with middleware for authentication. The storage layer (`server/storage.ts`) provides a clean interface for database operations.

### Data Storage
- **Database**: PostgreSQL with Drizzle ORM
- **Schema Location**: `shared/schema.ts` - shared between frontend and backend
- **Key Tables**:
  - `users` - User accounts (required for Replit Auth)
  - `sessions` - Session storage (required for Replit Auth)
  - `recipes` - Recipe content with title, content (markdown), and public/private flag
  - `chapters` - Organizational sections per user
  - `recipeChapters` - Many-to-many relationship between recipes and chapters

### Authentication
- Replit OIDC authentication flow
- Session-based auth with secure cookies
- User data synced from Replit profile (email, name, profile image)

### Key Design Decisions

**Monorepo Structure**: Client, server, and shared code coexist in one repository with path aliases (`@/` for client, `@shared/` for shared code).

**Shared Schema**: Database schema and Zod validation schemas are defined once in `shared/schema.ts` and used by both frontend and backend, ensuring type safety across the stack.

**Chapter-Based Organization**: Recipes can belong to multiple chapters (like tags but called "chapters" to match cookbook metaphor). This provides flexible organization without rigid folder hierarchies.

**Plain Text Recipes**: Recipe content is stored as markdown/plain text, allowing users to paste recipes from anywhere without complex structured data entry.

## External Dependencies

### Database
- PostgreSQL (provisioned via Replit)
- Connection via `DATABASE_URL` environment variable

### Authentication
- Replit OpenID Connect provider
- Required environment variables: `REPL_ID`, `ISSUER_URL`, `SESSION_SECRET`

### Frontend Libraries
- Radix UI primitives for accessible components
- Lucide React for icons
- date-fns for date formatting
- embla-carousel-react for carousels
- react-day-picker for calendar components
- vaul for drawer components
- cmdk for command palette

### Build & Development
- Vite with React plugin
- esbuild for server bundling
- TypeScript for type checking
- Drizzle Kit for database migrations (`npm run db:push`)