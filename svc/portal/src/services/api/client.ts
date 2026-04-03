"use client";

import { env } from "@/lib/env";

class ApiService {
  private baseURL: string;

  constructor(baseURL: string) {
    this.baseURL = baseURL;
  }

  getUrl(endpoint: string): string {
    return `${this.baseURL}${endpoint}`;
  }
}

export const apiService = new ApiService(env.NEXT_PUBLIC_API_URL);