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



export type listGroupsResponse200 = {
  data: void
  status: 200
}

export type listGroupsResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listGroupsResponse500 = {
  data: ProblemSchema
  status: 500
}

export type listGroupsResponseSuccess = (listGroupsResponse200) & {
  headers: Headers;
};
export type listGroupsResponseError = (listGroupsResponse401 | listGroupsResponse500) & {
  headers: Headers;
};

export type listGroupsResponse = (listGroupsResponseSuccess | listGroupsResponseError)

export const getListGroupsUrl = () => {




  return `/api/v1/groups`
}

/**
 * @summary Get a list of groups
 */
export const listGroups = async ( options?: RequestInit): Promise<listGroupsResponse> => {

  return customFetch<listGroupsResponse>(getListGroupsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}




export const getListGroupsMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listGroups>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof listGroups>>, TError,void, TContext> => {

const mutationKey = ['listGroups'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof listGroups>>, void> = () => {


          return  listGroups(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type ListGroupsMutationResult = NonNullable<Awaited<ReturnType<typeof listGroups>>>

    export type ListGroupsMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get a list of groups
 */
export const createListGroups = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listGroups>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof listGroups>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getListGroupsMutationOptions(options?.()) }), queryClient);
    }
    export type createGroupResponse200 = {
  data: void
  status: 200
}

export type createGroupResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createGroupResponse500 = {
  data: ProblemSchema
  status: 500
}

export type createGroupResponseSuccess = (createGroupResponse200) & {
  headers: Headers;
};
export type createGroupResponseError = (createGroupResponse401 | createGroupResponse500) & {
  headers: Headers;
};

export type createGroupResponse = (createGroupResponseSuccess | createGroupResponseError)

export const getCreateGroupUrl = () => {




  return `/api/v1/groups`
}

/**
 * @summary Create a new group
 */
export const createGroup = async ( options?: RequestInit): Promise<createGroupResponse> => {

  return customFetch<createGroupResponse>(getCreateGroupUrl(),
  {
    ...options,
    method: 'POST'


  }
);}





export const getCreateGroupQueryKey = () => {
    return [
    'POST', `/api/v1/groups`
    ] as const;
    }


export const getCreateGroupQueryOptions = <TData = Awaited<ReturnType<typeof createGroup>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getCreateGroupQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof createGroup>>> = ({ signal }) => createGroup({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof createGroup>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type CreateGroupQueryResult = NonNullable<Awaited<ReturnType<typeof createGroup>>>
export type CreateGroupQueryError = ErrorType<ProblemSchema>


/**
 * @summary Create a new group
 */

export function createCreateGroup<TData = Awaited<ReturnType<typeof createGroup>>, TError = ErrorType<ProblemSchema>>(
  options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getCreateGroupQueryOptions(options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type getGroupResponse200 = {
  data: void
  status: 200
}

export type getGroupResponse401 = {
  data: ProblemSchema
  status: 401
}

export type getGroupResponse404 = {
  data: ProblemSchema
  status: 404
}

export type getGroupResponse500 = {
  data: ProblemSchema
  status: 500
}

export type getGroupResponseSuccess = (getGroupResponse200) & {
  headers: Headers;
};
export type getGroupResponseError = (getGroupResponse401 | getGroupResponse404 | getGroupResponse500) & {
  headers: Headers;
};

export type getGroupResponse = (getGroupResponseSuccess | getGroupResponseError)

export const getGetGroupUrl = (id: string,) => {




  return `/api/v1/groups/${id}`
}

/**
 * @summary Get a specific group
 */
export const getGroup = async (id: string, options?: RequestInit): Promise<getGroupResponse> => {

  return customFetch<getGroupResponse>(getGetGroupUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}




export const getGetGroupMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getGroup>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof getGroup>>, TError,{id: string}, TContext> => {

const mutationKey = ['getGroup'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof getGroup>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  getGroup(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type GetGroupMutationResult = NonNullable<Awaited<ReturnType<typeof getGroup>>>

    export type GetGroupMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get a specific group
 */
export const createGetGroup = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getGroup>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof getGroup>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getGetGroupMutationOptions(options?.()) }), queryClient);
    }
    export type deleteGroupResponse200 = {
  data: void
  status: 200
}

export type deleteGroupResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteGroupResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteGroupResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteGroupResponseSuccess = (deleteGroupResponse200) & {
  headers: Headers;
};
export type deleteGroupResponseError = (deleteGroupResponse401 | deleteGroupResponse404 | deleteGroupResponse500) & {
  headers: Headers;
};

export type deleteGroupResponse = (deleteGroupResponseSuccess | deleteGroupResponseError)

export const getDeleteGroupUrl = (id: string,) => {




  return `/api/v1/groups/${id}`
}

/**
 * @summary Delete a group
 */
export const deleteGroup = async (id: string, options?: RequestInit): Promise<deleteGroupResponse> => {

  return customFetch<deleteGroupResponse>(getDeleteGroupUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}





export const getDeleteGroupQueryKey = (id: string,) => {
    return [
    'DELETE', `/api/v1/groups/${id}`
    ] as const;
    }


export const getDeleteGroupQueryOptions = <TData = Awaited<ReturnType<typeof deleteGroup>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getDeleteGroupQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof deleteGroup>>> = ({ signal }) => deleteGroup(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof deleteGroup>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type DeleteGroupQueryResult = NonNullable<Awaited<ReturnType<typeof deleteGroup>>>
export type DeleteGroupQueryError = ErrorType<ProblemSchema>


/**
 * @summary Delete a group
 */

export function createDeleteGroup<TData = Awaited<ReturnType<typeof deleteGroup>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getDeleteGroupQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type updateGroupResponse200 = {
  data: void
  status: 200
}

export type updateGroupResponse401 = {
  data: ProblemSchema
  status: 401
}

export type updateGroupResponse404 = {
  data: ProblemSchema
  status: 404
}

export type updateGroupResponse500 = {
  data: ProblemSchema
  status: 500
}

export type updateGroupResponseSuccess = (updateGroupResponse200) & {
  headers: Headers;
};
export type updateGroupResponseError = (updateGroupResponse401 | updateGroupResponse404 | updateGroupResponse500) & {
  headers: Headers;
};

export type updateGroupResponse = (updateGroupResponseSuccess | updateGroupResponseError)

export const getUpdateGroupUrl = (id: string,) => {




  return `/api/v1/groups/${id}`
}

/**
 * @summary Update a specific group
 */
export const updateGroup = async (id: string, options?: RequestInit): Promise<updateGroupResponse> => {

  return customFetch<updateGroupResponse>(getUpdateGroupUrl(id),
  {
    ...options,
    method: 'PATCH'


  }
);}





export const getUpdateGroupQueryKey = (id: string,) => {
    return [
    'PATCH', `/api/v1/groups/${id}`
    ] as const;
    }


export const getUpdateGroupQueryOptions = <TData = Awaited<ReturnType<typeof updateGroup>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getUpdateGroupQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof updateGroup>>> = ({ signal }) => updateGroup(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof updateGroup>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type UpdateGroupQueryResult = NonNullable<Awaited<ReturnType<typeof updateGroup>>>
export type UpdateGroupQueryError = ErrorType<ProblemSchema>


/**
 * @summary Update a specific group
 */

export function createUpdateGroup<TData = Awaited<ReturnType<typeof updateGroup>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getUpdateGroupQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






