import Image from "next/image";

export default function Header() {
  return (
    <header className="flex justify-between mx-auto max-w-[1024px] p-4  border-b">
      <div className="flex items-center">
        <Image src="/lerpz.svg" alt="Lerpz Logo" width={32} height={32} />
        <h1 className="ml-4 text-2xl font-bold">Lerpz AI</h1>
      </div>
      <div>SOLUTION</div>
      <div>Login</div>
    </header>
  );
}
