import { UserMenu } from "../UserMenu";

export default function UserMenuExample() {
  return (
    <UserMenu
      username="Jane Smith"
      initials="JS"
      onProfile={() => console.log("Profile")}
      onSettings={() => console.log("Settings")}
      onLogout={() => console.log("Logout")}
    />
  );
}
