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





export const getStartSessionQueryKey = () => {
    return [
    'POST', `/api/v1/sessions`
    ] as const;
    }


export const getStartSessionQueryOptions = <TData = Awaited<ReturnType<typeof startSession>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof startSession>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getStartSessionQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof startSession>>> = ({ signal }) => startSession({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof startSession>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type StartSessionQueryResult = NonNullable<Awaited<ReturnType<typeof startSession>>>
export type StartSessionQueryError = ErrorType<ProblemSchema>


/**
 * @summary Start a new session
 */

export function createStartSession<TData = Awaited<ReturnType<typeof startSession>>, TError = ErrorType<ProblemSchema>>(
  options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof startSession>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getStartSessionQueryOptions(options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
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





export const getDeleteSessionQueryKey = (id: string,) => {
    return [
    'DELETE', `/api/v1/sessions/${id}`
    ] as const;
    }


export const getDeleteSessionQueryOptions = <TData = Awaited<ReturnType<typeof deleteSession>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteSession>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getDeleteSessionQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof deleteSession>>> = ({ signal }) => deleteSession(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof deleteSession>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type DeleteSessionQueryResult = NonNullable<Awaited<ReturnType<typeof deleteSession>>>
export type DeleteSessionQueryError = ErrorType<ProblemSchema>


/**
 * @summary Delete a specific session
 */

export function createDeleteSession<TData = Awaited<ReturnType<typeof deleteSession>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteSession>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getDeleteSessionQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
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





export const getStopSessionQueryKey = (id: string,) => {
    return [
    'POST', `/api/v1/sessions/${id}/stop`
    ] as const;
    }


export const getStopSessionQueryOptions = <TData = Awaited<ReturnType<typeof stopSession>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof stopSession>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getStopSessionQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof stopSession>>> = ({ signal }) => stopSession(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof stopSession>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type StopSessionQueryResult = NonNullable<Awaited<ReturnType<typeof stopSession>>>
export type StopSessionQueryError = ErrorType<ProblemSchema>


/**
 * @summary Stop a specific session
 */

export function createStopSession<TData = Awaited<ReturnType<typeof stopSession>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof stopSession>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getStopSessionQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






