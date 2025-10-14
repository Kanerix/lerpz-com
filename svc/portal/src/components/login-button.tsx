import { signIn } from "@lerpz/lib/auth";

export default function LoginButton() {
  const handleForm = async () => {
    "use server";
    return await signIn("microsoft-entra-id", {
      redirectTo: "/",
    });
  };

  return (
    <form action={handleForm}>
      <button type="submit">login</button>
    </form>
  );
}
