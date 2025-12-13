# Design Guidelines: Recipe Management Platform

## Design Approach
**Hybrid Approach**: Drawing inspiration from **Notion** (organizational clarity) and **NYT Cooking** (recipe-focused UX) with **Material Design** principles for forms and data presentation.

Core philosophy: Clean, content-first design that prioritizes recipe readability while enabling effortless organization through visual hierarchy and intuitive navigation.

---

## Typography System

**Font Families**:
- Primary: Inter (UI, navigation, labels)
- Secondary: Georgia (recipe titles, headings - adds warmth)
- Monospace: JetBrains Mono (recipe content/plain text display)

**Hierarchy**:
- Page titles: text-4xl, font-bold
- Recipe titles: text-3xl, font-serif
- Section headers: text-xl, font-semibold
- Body text: text-base
- Labels/metadata: text-sm, font-medium
- Recipe content: text-base, font-mono, leading-relaxed

---

## Layout System

**Spacing Primitives**: Use Tailwind units of **2, 4, 6, and 8** consistently
- Component padding: p-4 or p-6
- Section spacing: gap-6 or gap-8
- Margins: m-4, m-6, m-8
- Card spacing: space-y-4

**Grid Structure**:
- Dashboard: 3-column grid (lg:grid-cols-3) for recipe cards
- Recipe book view: 2-column masonry grid (md:grid-cols-2)
- Single recipe: Single column, max-w-3xl centered
- Sidebar navigation: Fixed 64 width on desktop

---

## Core Components

### Navigation
- Persistent left sidebar (desktop): User profile at top, main navigation, tag list at bottom
- Mobile: Bottom navigation bar with hamburger menu for tags
- Top bar: Search, "New Recipe" button, user avatar

### Recipe Cards
- Rounded corners (rounded-lg)
- Title, author, tag chips, privacy indicator
- Hover state with subtle elevation
- Public/private badge (top-right corner)

### Recipe Editor
- Split view: Plain text editor (left) | Live preview (right) on desktop
- Mobile: Tabbed interface (Edit/Preview)
- Toolbar: Save, Delete, Visibility toggle, Tags selector
- Monospace font for editing area with line numbers

### Recipe Book View
- Header: Book name (tag name), recipe count, filter options
- Cards display in masonry grid
- Empty state: Illustration + "Add your first recipe"

### Print View Modal
- Full-screen overlay with SVG rendering
- Clean print controls: Download SVG, Print button
- Close button (top-right)

### Tag System
- Inline tag chips with rounded-full styling
- Tag selector: Dropdown with create new option
- Tag management: Editable list in settings

### Forms & Inputs
- Clean Material Design-inspired inputs
- Floating labels or clear placeholders
- Validation states with inline messages
- Toggle switches for public/private

---

## Page Layouts

### Dashboard (Home)
- Welcome header with user's name
- Quick stats: Total recipes, public recipes, tags count
- Recent recipes grid (3 columns)
- "Browse Public Recipes" section below

### My Recipes
- Filter bar: All/Public/Private, sort options
- Recipe grid with cards
- Floating "+" button (bottom-right)

### Recipe Book (Tag View)
- Tag header with edit/delete options
- Filtered recipe grid
- Add recipe button

### Public Recipes Browse
- Hero section: "Discover Recipes from Our Community"
- Search bar prominent
- Grid of public recipe cards from all users

### Single Recipe View
- Recipe title (large, serif)
- Author info with avatar
- Tag chips
- Plain text recipe content in monospace
- Action buttons: Edit (own recipes), Add to Book, Print
- Print button triggers WASM SVG generation

### User Profile
- Avatar, username, bio
- Stats: Recipes created, public recipes
- Recipe grid

---

## Interactions & States

**Animations**: Minimal, purposeful
- Card hover: Subtle shadow increase (transition-shadow duration-200)
- Button interactions: Scale on active (active:scale-95)
- Modal appearance: Fade-in backdrop, slide-down content
- Tag chips: Smooth color transitions

**Loading States**:
- Recipe cards: Skeleton loaders with pulse animation
- SVG generation: Spinner overlay with "Generating print view..."

---

## Accessibility
- Keyboard navigation throughout
- ARIA labels for icons and buttons
- Focus visible states (ring-2 ring-offset-2)
- Screen reader-friendly recipe structure
- High contrast text throughout

---

## Images

**Hero Section**: Not applicable (utility-focused app)

**User-Generated Content**:
- User avatars: Circular, 40x40px (profile), 32x32px (cards)
- Placeholder avatar: Initials on solid background

**Empty States**:
- Illustrated empty state graphics for:
  - No recipes yet
  - No public recipes found
  - Empty recipe book
- Simple, friendly line illustrations

**Recipe Cards**: No images (text-only recipes per requirements), visual interest through typography and tag chips