"use client";

import { Button } from "@lerpz/ui/components/button";
import { Input } from "@lerpz/ui/components/input";
import { TextLoop } from "@lerpz/ui/components/text-loop";
import { motion } from "motion/react";
import { MdArrowForward, MdAutoAwesome } from "react-icons/md";

export default function HeroSection() {
  return (
    <section className="relative flex flex-col items-center justify-center gap-8 py-16 md:py-24 text-center">
      {/* Eyebrow badge */}
      <motion.div
        initial={{ opacity: 0, y: 12 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.4 }}
        className="inline-flex items-center gap-2 rounded-full border border-primary/20 bg-primary/10 px-4 py-1.5 text-sm font-medium text-primary"
      >
        <MdAutoAwesome className="size-4 shrink-0" />
        <span>Your organisation's AI portal</span>
      </motion.div>

      {/* Headline */}
      <motion.h1
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.45, delay: 0.08 }}
        className="max-w-4xl text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl xl:text-7xl"
      >
        Unlock AI for{" "}
        <span className="inline-flex h-[1.2em] items-center text-primary">
          <TextLoop interval={2.8}>
            <span>your team</span>
            <span>your workflow</span>
            <span>your business</span>
          </TextLoop>
        </span>
      </motion.h1>

      {/* Sub-headline */}
      <motion.p
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.45, delay: 0.16 }}
        className="max-w-2xl text-lg text-muted-foreground md:text-xl"
      >
        Access a curated ecosystem of AI tools, resources, and expertise to
        drive innovation and efficiency across your organisation — all in one
        place.
      </motion.p>

      {/* CTA row */}
      <motion.div
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.45, delay: 0.24 }}
        className="flex w-full max-w-md flex-col gap-3 sm:flex-row"
      >
        <Input
          type="email"
          placeholder="Enter your work email…"
          className="h-11 flex-1 text-base"
        />
        <Button size="lg" className="shrink-0 gap-2">
          Get early access
          <MdArrowForward className="size-4" />
        </Button>
      </motion.div>

      {/* Social proof / trust strip */}
      <motion.p
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.5, delay: 0.36 }}
        className="text-xs text-muted-foreground"
      >
        No credit card required · Invite-only beta · Enterprise-ready
      </motion.p>
    </section>
  );
}