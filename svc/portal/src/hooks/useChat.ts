import useSWR from "swr";
import { apiService } from "@/services/chat";
import { authenticatedFetch } from "@/utils/fetch";

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
  const { data, error, isLoading, mutate } = useSWR(apiKeys.chat(), fetcher);

  return {
    data,
    isLoading,
    error,
    mutate,
  };
}
