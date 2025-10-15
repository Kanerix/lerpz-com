import { Button } from "@lerpz/ui/components/button";
import { signOut } from "@/lib/auth";

export default function LogoutButton() {
  const handleLogout = async () => {
    "use server";
    return await signOut({
      redirectTo: "/login",
    });
  };

  return (
    <form action={handleLogout}>
      <Button type="submit">logout</Button>
    </form>
  );
}
