import { useRouter } from "next/navigation";
import { signIn } from "@/lib/auth-client";

export default function LoginPage() {
  const router = useRouter();

  const handleLogin = async () => {
    await signIn.social({
      provider: "microsoft",
      callbackURL: "/",
      fetchOptions: {
        onSuccess: () => {
          router.push("/login");
        },
      },
    });
  };

  return (
    <div>
      <button type="button" onClick={handleLogin}>
        Sign In with Microsoft
      </button>
      ;
    </div>
  );
}
