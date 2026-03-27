import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";
import { MdConstruction, MdDescription, MdSettings } from "react-icons/md";
import Link from "next/link";
import { usePathname } from "next/navigation";
import type { ComponentPropsWithoutRef } from "react";

type NavSecondaryProps = ComponentPropsWithoutRef<typeof SidebarGroup>;

const pages = [
  {
    title: "Docs",
    url: "/docs",
    icon: MdDescription,
  },
  {
    title: "Settings",
    url: "/settings",
    icon: MdSettings,
  },
  {
    title: "Roadmap",
    url: "/roadmap",
    icon: MdConstruction,
  },
];

export default function NavSecondary({ ...props }: NavSecondaryProps) {
  const pathname = usePathname();

  return (
    <SidebarGroup {...props}>
      <SidebarGroupContent>
        <SidebarMenu>
          {pages.map((item) => (
            <SidebarMenuItem key={item.title}>
              <SidebarMenuButton
                isActive={item.url === pathname}
                render={
                  <Link href={item.url}>
                    <item.icon />
                    <span>{item.title}</span>
                  </Link>
                }
              ></SidebarMenuButton>
            </SidebarMenuItem>
          ))}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
