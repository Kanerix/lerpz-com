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



export type createAgentResponse200 = {
  data: void
  status: 200
}

export type createAgentResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createAgentResponse500 = {
  data: ProblemSchema
  status: 500
}

export type createAgentResponseSuccess = (createAgentResponse200) & {
  headers: Headers;
};
export type createAgentResponseError = (createAgentResponse401 | createAgentResponse500) & {
  headers: Headers;
};

export type createAgentResponse = (createAgentResponseSuccess | createAgentResponseError)

export const getCreateAgentUrl = () => {




  return `/api/v1/agents`
}

/**
 * @summary Create a new agent
 */
export const createAgent = async ( options?: RequestInit): Promise<createAgentResponse> => {

  return customFetch<createAgentResponse>(getCreateAgentUrl(),
  {
    ...options,
    method: 'POST'


  }
);}





export const getCreateAgentQueryKey = () => {
    return [
    'POST', `/api/v1/agents`
    ] as const;
    }


export const getCreateAgentQueryOptions = <TData = Awaited<ReturnType<typeof createAgent>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createAgent>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getCreateAgentQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof createAgent>>> = ({ signal }) => createAgent({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof createAgent>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type CreateAgentQueryResult = NonNullable<Awaited<ReturnType<typeof createAgent>>>
export type CreateAgentQueryError = ErrorType<ProblemSchema>


/**
 * @summary Create a new agent
 */

export function createCreateAgent<TData = Awaited<ReturnType<typeof createAgent>>, TError = ErrorType<ProblemSchema>>(
  options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createAgent>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getCreateAgentQueryOptions(options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type deleteAgentResponse200 = {
  data: void
  status: 200
}

export type deleteAgentResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteAgentResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteAgentResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteAgentResponseSuccess = (deleteAgentResponse200) & {
  headers: Headers;
};
export type deleteAgentResponseError = (deleteAgentResponse401 | deleteAgentResponse404 | deleteAgentResponse500) & {
  headers: Headers;
};

export type deleteAgentResponse = (deleteAgentResponseSuccess | deleteAgentResponseError)

export const getDeleteAgentUrl = (id: string,) => {




  return `/api/v1/agents/${id}`
}

/**
 * @summary Delete a specific agent
 */
export const deleteAgent = async (id: string, options?: RequestInit): Promise<deleteAgentResponse> => {

  return customFetch<deleteAgentResponse>(getDeleteAgentUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}





export const getDeleteAgentQueryKey = (id: string,) => {
    return [
    'DELETE', `/api/v1/agents/${id}`
    ] as const;
    }


export const getDeleteAgentQueryOptions = <TData = Awaited<ReturnType<typeof deleteAgent>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteAgent>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getDeleteAgentQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof deleteAgent>>> = ({ signal }) => deleteAgent(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof deleteAgent>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type DeleteAgentQueryResult = NonNullable<Awaited<ReturnType<typeof deleteAgent>>>
export type DeleteAgentQueryError = ErrorType<ProblemSchema>


/**
 * @summary Delete a specific agent
 */

export function createDeleteAgent<TData = Awaited<ReturnType<typeof deleteAgent>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteAgent>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getDeleteAgentQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type updateAgentResponse200 = {
  data: void
  status: 200
}

export type updateAgentResponse401 = {
  data: ProblemSchema
  status: 401
}

export type updateAgentResponse404 = {
  data: ProblemSchema
  status: 404
}

export type updateAgentResponse500 = {
  data: ProblemSchema
  status: 500
}

export type updateAgentResponseSuccess = (updateAgentResponse200) & {
  headers: Headers;
};
export type updateAgentResponseError = (updateAgentResponse401 | updateAgentResponse404 | updateAgentResponse500) & {
  headers: Headers;
};

export type updateAgentResponse = (updateAgentResponseSuccess | updateAgentResponseError)

export const getUpdateAgentUrl = (id: string,) => {




  return `/api/v1/agents/${id}`
}

/**
 * @summary Update a specific agent
 */
export const updateAgent = async (id: string, options?: RequestInit): Promise<updateAgentResponse> => {

  return customFetch<updateAgentResponse>(getUpdateAgentUrl(id),
  {
    ...options,
    method: 'PATCH'


  }
);}





export const getUpdateAgentQueryKey = (id: string,) => {
    return [
    'PATCH', `/api/v1/agents/${id}`
    ] as const;
    }


export const getUpdateAgentQueryOptions = <TData = Awaited<ReturnType<typeof updateAgent>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateAgent>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getUpdateAgentQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof updateAgent>>> = ({ signal }) => updateAgent(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof updateAgent>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type UpdateAgentQueryResult = NonNullable<Awaited<ReturnType<typeof updateAgent>>>
export type UpdateAgentQueryError = ErrorType<ProblemSchema>


/**
 * @summary Update a specific agent
 */

export function createUpdateAgent<TData = Awaited<ReturnType<typeof updateAgent>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateAgent>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getUpdateAgentQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






