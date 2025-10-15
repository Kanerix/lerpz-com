import { Button } from "@lerpz/ui/components/button";
import { signIn } from "@/lib/auth";

export default function LoginButton() {
  const handleForm = async () => {
    "use server";
    return await signIn("microsoft-entra-id", {
      redirectTo: "/",
    });
  };

  return (
    <form action={handleForm}>
      <Button type="submit">login</Button>
    </form>
  );
}
