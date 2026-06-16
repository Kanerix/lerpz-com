// @ts-nocheck
import {
  createMutation,
  createQuery
} from '@tanstack/svelte-query';
import type {
  CreateMutationOptions,
  CreateMutationResult,
  CreateQueryOptions,
  CreateQueryResult,
  DataTag,
  MutationFunction,
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



export type listOrgsResponse200 = {
  data: void
  status: 200
}

export type listOrgsResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listOrgsResponse500 = {
  data: ProblemSchema
  status: 500
}

export type listOrgsResponseSuccess = (listOrgsResponse200) & {
  headers: Headers;
};
export type listOrgsResponseError = (listOrgsResponse401 | listOrgsResponse500) & {
  headers: Headers;
};

export type listOrgsResponse = (listOrgsResponseSuccess | listOrgsResponseError)

export const getListOrgsUrl = () => {




  return `/api/v1/orgs`
}

/**
 * @summary Get a list of organization
 */
export const listOrgs = async ( options?: RequestInit): Promise<listOrgsResponse> => {

  return customFetch<listOrgsResponse>(getListOrgsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}




export const getListOrgsMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listOrgs>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof listOrgs>>, TError,void, TContext> => {

const mutationKey = ['listOrgs'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof listOrgs>>, void> = () => {


          return  listOrgs(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type ListOrgsMutationResult = NonNullable<Awaited<ReturnType<typeof listOrgs>>>

    export type ListOrgsMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get a list of organization
 */
export const createListOrgs = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listOrgs>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof listOrgs>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getListOrgsMutationOptions(options?.()) }), queryClient);
    }
    export type createOrgResponse200 = {
  data: void
  status: 200
}

export type createOrgResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createOrgResponse500 = {
  data: ProblemSchema
  status: 500
}

export type createOrgResponseSuccess = (createOrgResponse200) & {
  headers: Headers;
};
export type createOrgResponseError = (createOrgResponse401 | createOrgResponse500) & {
  headers: Headers;
};

export type createOrgResponse = (createOrgResponseSuccess | createOrgResponseError)

export const getCreateOrgUrl = () => {




  return `/api/v1/orgs`
}

/**
 * @summary Create a new organization
 */
export const createOrg = async ( options?: RequestInit): Promise<createOrgResponse> => {

  return customFetch<createOrgResponse>(getCreateOrgUrl(),
  {
    ...options,
    method: 'POST'


  }
);}





export const getCreateOrgQueryKey = () => {
    return [
    'POST', `/api/v1/orgs`
    ] as const;
    }


export const getCreateOrgQueryOptions = <TData = Awaited<ReturnType<typeof createOrg>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createOrg>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getCreateOrgQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof createOrg>>> = ({ signal }) => createOrg({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof createOrg>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type CreateOrgQueryResult = NonNullable<Awaited<ReturnType<typeof createOrg>>>
export type CreateOrgQueryError = ErrorType<ProblemSchema>


/**
 * @summary Create a new organization
 */

export function createCreateOrg<TData = Awaited<ReturnType<typeof createOrg>>, TError = ErrorType<ProblemSchema>>(
  options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createOrg>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getCreateOrgQueryOptions(options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type getOrgResponse200 = {
  data: void
  status: 200
}

export type getOrgResponse401 = {
  data: ProblemSchema
  status: 401
}

export type getOrgResponse404 = {
  data: ProblemSchema
  status: 404
}

export type getOrgResponse500 = {
  data: ProblemSchema
  status: 500
}

export type getOrgResponseSuccess = (getOrgResponse200) & {
  headers: Headers;
};
export type getOrgResponseError = (getOrgResponse401 | getOrgResponse404 | getOrgResponse500) & {
  headers: Headers;
};

export type getOrgResponse = (getOrgResponseSuccess | getOrgResponseError)

export const getGetOrgUrl = (id: string,) => {




  return `/api/v1/orgs/${id}`
}

/**
 * @summary Get a specific organization
 */
export const getOrg = async (id: string, options?: RequestInit): Promise<getOrgResponse> => {

  return customFetch<getOrgResponse>(getGetOrgUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}




export const getGetOrgMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getOrg>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof getOrg>>, TError,{id: string}, TContext> => {

const mutationKey = ['getOrg'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof getOrg>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  getOrg(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type GetOrgMutationResult = NonNullable<Awaited<ReturnType<typeof getOrg>>>

    export type GetOrgMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get a specific organization
 */
export const createGetOrg = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getOrg>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof getOrg>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getGetOrgMutationOptions(options?.()) }), queryClient);
    }
    export type deleteOrgResponse200 = {
  data: void
  status: 200
}

export type deleteOrgResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteOrgResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteOrgResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteOrgResponseSuccess = (deleteOrgResponse200) & {
  headers: Headers;
};
export type deleteOrgResponseError = (deleteOrgResponse401 | deleteOrgResponse404 | deleteOrgResponse500) & {
  headers: Headers;
};

export type deleteOrgResponse = (deleteOrgResponseSuccess | deleteOrgResponseError)

export const getDeleteOrgUrl = (id: string,) => {




  return `/api/v1/orgs/${id}`
}

/**
 * @summary Delete a new organization
 */
export const deleteOrg = async (id: string, options?: RequestInit): Promise<deleteOrgResponse> => {

  return customFetch<deleteOrgResponse>(getDeleteOrgUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}





export const getDeleteOrgQueryKey = (id: string,) => {
    return [
    'DELETE', `/api/v1/orgs/${id}`
    ] as const;
    }


export const getDeleteOrgQueryOptions = <TData = Awaited<ReturnType<typeof deleteOrg>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteOrg>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getDeleteOrgQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof deleteOrg>>> = ({ signal }) => deleteOrg(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof deleteOrg>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type DeleteOrgQueryResult = NonNullable<Awaited<ReturnType<typeof deleteOrg>>>
export type DeleteOrgQueryError = ErrorType<ProblemSchema>


/**
 * @summary Delete a new organization
 */

export function createDeleteOrg<TData = Awaited<ReturnType<typeof deleteOrg>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteOrg>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getDeleteOrgQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type updateOrgResponse200 = {
  data: void
  status: 200
}

export type updateOrgResponse401 = {
  data: ProblemSchema
  status: 401
}

export type updateOrgResponse404 = {
  data: ProblemSchema
  status: 404
}

export type updateOrgResponse500 = {
  data: ProblemSchema
  status: 500
}

export type updateOrgResponseSuccess = (updateOrgResponse200) & {
  headers: Headers;
};
export type updateOrgResponseError = (updateOrgResponse401 | updateOrgResponse404 | updateOrgResponse500) & {
  headers: Headers;
};

export type updateOrgResponse = (updateOrgResponseSuccess | updateOrgResponseError)

export const getUpdateOrgUrl = (id: string,) => {




  return `/api/v1/orgs/${id}`
}

/**
 * @summary Update a specific organization
 */
export const updateOrg = async (id: string, options?: RequestInit): Promise<updateOrgResponse> => {

  return customFetch<updateOrgResponse>(getUpdateOrgUrl(id),
  {
    ...options,
    method: 'PATCH'


  }
);}





export const getUpdateOrgQueryKey = (id: string,) => {
    return [
    'PATCH', `/api/v1/orgs/${id}`
    ] as const;
    }


export const getUpdateOrgQueryOptions = <TData = Awaited<ReturnType<typeof updateOrg>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateOrg>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getUpdateOrgQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof updateOrg>>> = ({ signal }) => updateOrg(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof updateOrg>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type UpdateOrgQueryResult = NonNullable<Awaited<ReturnType<typeof updateOrg>>>
export type UpdateOrgQueryError = ErrorType<ProblemSchema>


/**
 * @summary Update a specific organization
 */

export function createUpdateOrg<TData = Awaited<ReturnType<typeof updateOrg>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateOrg>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getUpdateOrgQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






