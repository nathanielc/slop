import { EmptyState } from "../EmptyState";

export default function EmptyStateExample() {
  return (
    <EmptyState
      type="no-recipes"
      onAction={() => console.log("Create recipe clicked")}
    />
  );
}
