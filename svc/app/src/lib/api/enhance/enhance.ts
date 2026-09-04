// @ts-nocheck
import {
  createQuery
} from '@tanstack/svelte-query';
import type {
  CreateQueryOptions,
  CreateQueryResult,
  DataTag,
  QueryClient,
  QueryFunction,
  QueryKey
} from '@tanstack/svelte-query';

import type {
  EnhanceRequest,
  EnhanceResponse,
  ProblemSchema
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type enhanceChatPromptResponse200 = {
  data: EnhanceResponse
  status: 200
}

export type enhanceChatPromptResponse400 = {
  data: ProblemSchema
  status: 400
}

export type enhanceChatPromptResponse401 = {
  data: ProblemSchema
  status: 401
}

export type enhanceChatPromptResponse500 = {
  data: ProblemSchema
  status: 500
}

export type enhanceChatPromptResponseSuccess = (enhanceChatPromptResponse200) & {
  headers: Headers;
};
export type enhanceChatPromptResponseError = (enhanceChatPromptResponse400 | enhanceChatPromptResponse401 | enhanceChatPromptResponse500) & {
  headers: Headers;
};

export type enhanceChatPromptResponse = (enhanceChatPromptResponseSuccess | enhanceChatPromptResponseError)

export const getEnhanceChatPromptUrl = () => {




  return `/api/v1/enhance/chat`
}

/**
 * Rewrites a raw prompt into a clearer, more effective prompt for a conversational AI assistant.
 * @summary Enhance a chat prompt
 */
export const enhanceChatPrompt = async (enhanceRequest: EnhanceRequest, options?: RequestInit): Promise<enhanceChatPromptResponse> => {

  return customFetch<enhanceChatPromptResponse>(getEnhanceChatPromptUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(enhanceRequest)
  }
);}





export const getEnhanceChatPromptQueryKey = (enhanceRequest?: EnhanceRequest,) => {
    return [
    'POST', `/api/v1/enhance/chat`, enhanceRequest
    ] as const;
    }


export const getEnhanceChatPromptQueryOptions = <TData = Awaited<ReturnType<typeof enhanceChatPrompt>>, TError = ErrorType<ProblemSchema>>(enhanceRequest: EnhanceRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof enhanceChatPrompt>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getEnhanceChatPromptQueryKey(enhanceRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof enhanceChatPrompt>>> = ({ signal }) => enhanceChatPrompt(enhanceRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof enhanceChatPrompt>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type EnhanceChatPromptQueryResult = NonNullable<Awaited<ReturnType<typeof enhanceChatPrompt>>>
export type EnhanceChatPromptQueryError = ErrorType<ProblemSchema>


/**
 * @summary Enhance a chat prompt
 */

export function createEnhanceChatPrompt<TData = Awaited<ReturnType<typeof enhanceChatPrompt>>, TError = ErrorType<ProblemSchema>>(
 enhanceRequest: () =>  EnhanceRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof enhanceChatPrompt>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getEnhanceChatPromptQueryOptions(enhanceRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type enhanceImagePromptResponse200 = {
  data: EnhanceResponse
  status: 200
}

export type enhanceImagePromptResponse400 = {
  data: ProblemSchema
  status: 400
}

export type enhanceImagePromptResponse401 = {
  data: ProblemSchema
  status: 401
}

export type enhanceImagePromptResponse500 = {
  data: ProblemSchema
  status: 500
}

export type enhanceImagePromptResponseSuccess = (enhanceImagePromptResponse200) & {
  headers: Headers;
};
export type enhanceImagePromptResponseError = (enhanceImagePromptResponse400 | enhanceImagePromptResponse401 | enhanceImagePromptResponse500) & {
  headers: Headers;
};

export type enhanceImagePromptResponse = (enhanceImagePromptResponseSuccess | enhanceImagePromptResponseError)

export const getEnhanceImagePromptUrl = () => {




  return `/api/v1/enhance/image`
}

/**
 * Rewrites a raw prompt into a vivid, detailed prompt for a text-to-image generation model.
 * @summary Enhance an image prompt
 */
export const enhanceImagePrompt = async (enhanceRequest: EnhanceRequest, options?: RequestInit): Promise<enhanceImagePromptResponse> => {

  return customFetch<enhanceImagePromptResponse>(getEnhanceImagePromptUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(enhanceRequest)
  }
);}





export const getEnhanceImagePromptQueryKey = (enhanceRequest?: EnhanceRequest,) => {
    return [
    'POST', `/api/v1/enhance/image`, enhanceRequest
    ] as const;
    }


export const getEnhanceImagePromptQueryOptions = <TData = Awaited<ReturnType<typeof enhanceImagePrompt>>, TError = ErrorType<ProblemSchema>>(enhanceRequest: EnhanceRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof enhanceImagePrompt>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getEnhanceImagePromptQueryKey(enhanceRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof enhanceImagePrompt>>> = ({ signal }) => enhanceImagePrompt(enhanceRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof enhanceImagePrompt>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type EnhanceImagePromptQueryResult = NonNullable<Awaited<ReturnType<typeof enhanceImagePrompt>>>
export type EnhanceImagePromptQueryError = ErrorType<ProblemSchema>


/**
 * @summary Enhance an image prompt
 */

export function createEnhanceImagePrompt<TData = Awaited<ReturnType<typeof enhanceImagePrompt>>, TError = ErrorType<ProblemSchema>>(
 enhanceRequest: () =>  EnhanceRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof enhanceImagePrompt>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getEnhanceImagePromptQueryOptions(enhanceRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type enhanceVideoPromptResponse200 = {
  data: EnhanceResponse
  status: 200
}

export type enhanceVideoPromptResponse400 = {
  data: ProblemSchema
  status: 400
}

export type enhanceVideoPromptResponse401 = {
  data: ProblemSchema
  status: 401
}

export type enhanceVideoPromptResponse500 = {
  data: ProblemSchema
  status: 500
}

export type enhanceVideoPromptResponseSuccess = (enhanceVideoPromptResponse200) & {
  headers: Headers;
};
export type enhanceVideoPromptResponseError = (enhanceVideoPromptResponse400 | enhanceVideoPromptResponse401 | enhanceVideoPromptResponse500) & {
  headers: Headers;
};

export type enhanceVideoPromptResponse = (enhanceVideoPromptResponseSuccess | enhanceVideoPromptResponseError)

export const getEnhanceVideoPromptUrl = () => {




  return `/api/v1/enhance/video`
}

/**
 * Rewrites a raw prompt into a vivid, cinematic prompt for a text-to-video generation model.
 * @summary Enhance a video prompt
 */
export const enhanceVideoPrompt = async (enhanceRequest: EnhanceRequest, options?: RequestInit): Promise<enhanceVideoPromptResponse> => {

  return customFetch<enhanceVideoPromptResponse>(getEnhanceVideoPromptUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(enhanceRequest)
  }
);}





export const getEnhanceVideoPromptQueryKey = (enhanceRequest?: EnhanceRequest,) => {
    return [
    'POST', `/api/v1/enhance/video`, enhanceRequest
    ] as const;
    }


export const getEnhanceVideoPromptQueryOptions = <TData = Awaited<ReturnType<typeof enhanceVideoPrompt>>, TError = ErrorType<ProblemSchema>>(enhanceRequest: EnhanceRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof enhanceVideoPrompt>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getEnhanceVideoPromptQueryKey(enhanceRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof enhanceVideoPrompt>>> = ({ signal }) => enhanceVideoPrompt(enhanceRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof enhanceVideoPrompt>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type EnhanceVideoPromptQueryResult = NonNullable<Awaited<ReturnType<typeof enhanceVideoPrompt>>>
export type EnhanceVideoPromptQueryError = ErrorType<ProblemSchema>


/**
 * @summary Enhance a video prompt
 */

export function createEnhanceVideoPrompt<TData = Awaited<ReturnType<typeof enhanceVideoPrompt>>, TError = ErrorType<ProblemSchema>>(
 enhanceRequest: () =>  EnhanceRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof enhanceVideoPrompt>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getEnhanceVideoPromptQueryOptions(enhanceRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






