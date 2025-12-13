import { LoginCard } from "../LoginCard";

export default function LoginCardExample() {
  return <LoginCard onLogin={() => console.log("Login clicked")} />;
}
