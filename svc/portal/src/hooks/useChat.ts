import useSWR from "swr";
import { authenticatedFetch } from "@/lib/fetch";
import { apiService } from "@/services/chat";

export const apiKeys = {
  chat: () => apiService.getUrl("/chat/create"),
};

async function fetcher<T>(url: string): Promise<T> {
  const response = await authenticatedFetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      message: "Tell me a joke about programming!",
    }),
  });

  if (!response.ok) {
    const error = new Error("An error occurred while fetching the data.");
    throw error;
  }

  return response.json();
}

export function useChat() {
  return useSWR(apiKeys.chat(), fetcher);
}
