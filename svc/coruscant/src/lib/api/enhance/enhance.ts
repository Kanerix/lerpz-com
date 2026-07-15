// @ts-nocheck
import {
  createMutation
} from '@tanstack/svelte-query';
import type {
  CreateMutationOptions,
  CreateMutationResult,
  MutationFunction,
  QueryClient
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




export const getEnhanceChatPromptMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof enhanceChatPrompt>>, TError,{data: EnhanceRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof enhanceChatPrompt>>, TError,{data: EnhanceRequest}, TContext> => {

const mutationKey = ['enhanceChatPrompt'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};


      const mutationFn: MutationFunction<Awaited<ReturnType<typeof enhanceChatPrompt>>, {data: EnhanceRequest}> = (props) => {
          const {data} = props ?? {};

          return  enhanceChatPrompt(data,requestOptions)
        }




  return  { mutationFn, ...mutationOptions }}

    export type EnhanceChatPromptMutationResult = NonNullable<Awaited<ReturnType<typeof enhanceChatPrompt>>>
    export type EnhanceChatPromptMutationBody = EnhanceRequest
    export type EnhanceChatPromptMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Enhance a chat prompt
 */
export const createEnhanceChatPrompt = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof enhanceChatPrompt>>, TError,{data: EnhanceRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof enhanceChatPrompt>>,
        TError,
        {data: EnhanceRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getEnhanceChatPromptMutationOptions(options?.()) }), queryClient);
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




export const getEnhanceImagePromptMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof enhanceImagePrompt>>, TError,{data: EnhanceRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof enhanceImagePrompt>>, TError,{data: EnhanceRequest}, TContext> => {

const mutationKey = ['enhanceImagePrompt'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};


      const mutationFn: MutationFunction<Awaited<ReturnType<typeof enhanceImagePrompt>>, {data: EnhanceRequest}> = (props) => {
          const {data} = props ?? {};

          return  enhanceImagePrompt(data,requestOptions)
        }




  return  { mutationFn, ...mutationOptions }}

    export type EnhanceImagePromptMutationResult = NonNullable<Awaited<ReturnType<typeof enhanceImagePrompt>>>
    export type EnhanceImagePromptMutationBody = EnhanceRequest
    export type EnhanceImagePromptMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Enhance an image prompt
 */
export const createEnhanceImagePrompt = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof enhanceImagePrompt>>, TError,{data: EnhanceRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof enhanceImagePrompt>>,
        TError,
        {data: EnhanceRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getEnhanceImagePromptMutationOptions(options?.()) }), queryClient);
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




export const getEnhanceVideoPromptMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof enhanceVideoPrompt>>, TError,{data: EnhanceRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof enhanceVideoPrompt>>, TError,{data: EnhanceRequest}, TContext> => {

const mutationKey = ['enhanceVideoPrompt'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};


      const mutationFn: MutationFunction<Awaited<ReturnType<typeof enhanceVideoPrompt>>, {data: EnhanceRequest}> = (props) => {
          const {data} = props ?? {};

          return  enhanceVideoPrompt(data,requestOptions)
        }




  return  { mutationFn, ...mutationOptions }}

    export type EnhanceVideoPromptMutationResult = NonNullable<Awaited<ReturnType<typeof enhanceVideoPrompt>>>
    export type EnhanceVideoPromptMutationBody = EnhanceRequest
    export type EnhanceVideoPromptMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Enhance a video prompt
 */
export const createEnhanceVideoPrompt = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof enhanceVideoPrompt>>, TError,{data: EnhanceRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof enhanceVideoPrompt>>,
        TError,
        {data: EnhanceRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getEnhanceVideoPromptMutationOptions(options?.()) }), queryClient);
    }
