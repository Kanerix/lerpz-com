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
  ProblemSchema
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type startSessionResponse200 = {
  data: void
  status: 200
}

export type startSessionResponse401 = {
  data: ProblemSchema
  status: 401
}

export type startSessionResponse500 = {
  data: ProblemSchema
  status: 500
}

export type startSessionResponseSuccess = (startSessionResponse200) & {
  headers: Headers;
};
export type startSessionResponseError = (startSessionResponse401 | startSessionResponse500) & {
  headers: Headers;
};

export type startSessionResponse = (startSessionResponseSuccess | startSessionResponseError)

export const getStartSessionUrl = () => {




  return `/api/v1/sessions`
}

/**
 * @summary Start a new session
 */
export const startSession = async ( options?: RequestInit): Promise<startSessionResponse> => {

  return customFetch<startSessionResponse>(getStartSessionUrl(),
  {
    ...options,
    method: 'POST'


  }
);}




export const getStartSessionMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof startSession>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof startSession>>, TError,void, TContext> => {

const mutationKey = ['startSession'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof startSession>>, void> = () => {


          return  startSession(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type StartSessionMutationResult = NonNullable<Awaited<ReturnType<typeof startSession>>>

    export type StartSessionMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Start a new session
 */
export const createStartSession = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof startSession>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof startSession>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getStartSessionMutationOptions(options?.()) }), queryClient);
    }
    export type deleteSessionResponse200 = {
  data: void
  status: 200
}

export type deleteSessionResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteSessionResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteSessionResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteSessionResponseSuccess = (deleteSessionResponse200) & {
  headers: Headers;
};
export type deleteSessionResponseError = (deleteSessionResponse401 | deleteSessionResponse404 | deleteSessionResponse500) & {
  headers: Headers;
};

export type deleteSessionResponse = (deleteSessionResponseSuccess | deleteSessionResponseError)

export const getDeleteSessionUrl = (id: string,) => {




  return `/api/v1/sessions/${id}`
}

/**
 * @summary Delete a specific session
 */
export const deleteSession = async (id: string, options?: RequestInit): Promise<deleteSessionResponse> => {

  return customFetch<deleteSessionResponse>(getDeleteSessionUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}




export const getDeleteSessionMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof deleteSession>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof deleteSession>>, TError,{id: string}, TContext> => {

const mutationKey = ['deleteSession'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof deleteSession>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  deleteSession(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type DeleteSessionMutationResult = NonNullable<Awaited<ReturnType<typeof deleteSession>>>

    export type DeleteSessionMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Delete a specific session
 */
export const createDeleteSession = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof deleteSession>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof deleteSession>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getDeleteSessionMutationOptions(options?.()) }), queryClient);
    }
    export type stopSessionResponse200 = {
  data: void
  status: 200
}

export type stopSessionResponse401 = {
  data: ProblemSchema
  status: 401
}

export type stopSessionResponse404 = {
  data: ProblemSchema
  status: 404
}

export type stopSessionResponse500 = {
  data: ProblemSchema
  status: 500
}

export type stopSessionResponseSuccess = (stopSessionResponse200) & {
  headers: Headers;
};
export type stopSessionResponseError = (stopSessionResponse401 | stopSessionResponse404 | stopSessionResponse500) & {
  headers: Headers;
};

export type stopSessionResponse = (stopSessionResponseSuccess | stopSessionResponseError)

export const getStopSessionUrl = (id: string,) => {




  return `/api/v1/sessions/${id}/stop`
}

/**
 * @summary Stop a specific session
 */
export const stopSession = async (id: string, options?: RequestInit): Promise<stopSessionResponse> => {

  return customFetch<stopSessionResponse>(getStopSessionUrl(id),
  {
    ...options,
    method: 'POST'


  }
);}




export const getStopSessionMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof stopSession>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof stopSession>>, TError,{id: string}, TContext> => {

const mutationKey = ['stopSession'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof stopSession>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  stopSession(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type StopSessionMutationResult = NonNullable<Awaited<ReturnType<typeof stopSession>>>

    export type StopSessionMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Stop a specific session
 */
export const createStopSession = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof stopSession>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof stopSession>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getStopSessionMutationOptions(options?.()) }), queryClient);
    }
