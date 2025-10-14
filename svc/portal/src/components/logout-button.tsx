import { signOut } from "@lerpz/lib/auth";

export default function LogoutButton() {
  const handleForm = async () => {
    "use server";
    return await signOut({
      redirectTo: "/login",
    });
  };

  return (
    <form action={handleForm}>
      <button type="submit">logout</button>
    </form>
  );
}
