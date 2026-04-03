"use client";

import { Button } from "@lerpz/ui/components/button";
import { TextLoop } from "@lerpz/ui/components/text-loop";
import { motion } from "motion/react";
import Link from "next/link";
import { MdArrowForward } from "react-icons/md";
import LoginButton from "../login-button";

export default function HeroSection() {
  return (
    <section className="flex flex-col gap-6 py-16 md:py-24">
      <motion.p
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.35 }}
        className="text-sm font-medium text-muted-foreground uppercase tracking-widest"
      >
        Lerpz AI Portal
      </motion.p>

      <motion.h1
        initial={{ opacity: 0, y: 14 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.4, delay: 0.06 }}
        className="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl xl:text-7xl max-w-3xl"
      >
        AI tools for{" "}
        <span className="inline-flex h-[1.15em] items-end text-primary">
          <TextLoop interval={3}>
            <span>your daily work.</span>
            <span>your whole team.</span>
            <span>your organisation.</span>
          </TextLoop>
        </span>
      </motion.h1>

      <motion.p
        initial={{ opacity: 0, y: 14 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.4, delay: 0.12 }}
        className="max-w-xl text-lg text-muted-foreground leading-relaxed"
      >
        Sign in with your work account to access AI chat, image generation, and
        more — centralised and maintained for everyone in the organisation.
      </motion.p>

      <motion.div
        initial={{ opacity: 0, y: 14 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.4, delay: 0.18 }}
        className="flex items-center gap-3 pt-2"
      >
        <LoginButton size="lg">
          Sign in
          <MdArrowForward className="size-4" />
        </LoginButton>
        <Button variant="ghost" size="lg" nativeButton={false} render={<Link href="/docs" />}>
          Documentation
        </Button>
      </motion.div>
    </section>
  );
}