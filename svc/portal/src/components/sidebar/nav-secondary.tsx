import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@lerpz/ui/components/sidebar";
import { Construction, Settings2 } from "lucide-react";
import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";

type NavSecondaryProps = ComponentPropsWithoutRef<typeof SidebarGroup>;

const pages = [
  {
    title: "Settings",
    url: "/settings",
    icon: Settings2,
  },
  {
    title: "Roadmap",
    url: "/roadmap",
    icon: Construction,
  },
];

export default function NavSecondary({ ...props }: NavSecondaryProps) {
  return (
    <SidebarGroup {...props}>
      <SidebarGroupContent>
        <SidebarMenu>
          {pages.map((item) => (
            <SidebarMenuItem key={item.title}>
              <SidebarMenuButton
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
