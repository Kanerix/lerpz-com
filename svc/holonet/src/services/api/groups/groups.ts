// @ts-nocheck
import {
  useMutation,
  useQuery
} from '@tanstack/react-query';
import type {
  DataTag,
  DefinedInitialDataOptions,
  DefinedUseQueryResult,
  MutationFunction,
  QueryClient,
  QueryFunction,
  QueryKey,
  UndefinedInitialDataOptions,
  UseMutationOptions,
  UseMutationResult,
  UseQueryOptions,
  UseQueryResult
} from '@tanstack/react-query';

import type {
  ProblemSchema
} from '../models';

import { customFetch } from '../../../lib/orval-mutator';
import type { ErrorType } from '../../../lib/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



/**
 * @summary Get a list of groups
 */
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

export const listGroups = async ( options?: RequestInit): Promise<listGroupsResponse> => {

  return customFetch<listGroupsResponse>(getListGroupsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}





export const getListGroupsQueryKey = () => {
    return [
    `/api/v1/groups`
    ] as const;
    }


export const getListGroupsQueryOptions = <TData = Awaited<ReturnType<typeof listGroups>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listGroups>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getListGroupsQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof listGroups>>> = ({ signal }) => listGroups({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as UseQueryOptions<Awaited<ReturnType<typeof listGroups>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type ListGroupsQueryResult = NonNullable<Awaited<ReturnType<typeof listGroups>>>
export type ListGroupsQueryError = ErrorType<ProblemSchema>


export function useListGroups<TData = Awaited<ReturnType<typeof listGroups>>, TError = ErrorType<ProblemSchema>>(
  options: { query:Partial<UseQueryOptions<Awaited<ReturnType<typeof listGroups>>, TError, TData>> & Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof listGroups>>,
          TError,
          Awaited<ReturnType<typeof listGroups>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useListGroups<TData = Awaited<ReturnType<typeof listGroups>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listGroups>>, TError, TData>> & Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof listGroups>>,
          TError,
          Awaited<ReturnType<typeof listGroups>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useListGroups<TData = Awaited<ReturnType<typeof listGroups>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listGroups>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
/**
 * @summary Get a list of groups
 */

export function useListGroups<TData = Awaited<ReturnType<typeof listGroups>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listGroups>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
 ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {

  const queryOptions = getListGroupsQueryOptions(options)

  const query = useQuery(queryOptions, queryClient) as  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return { ...query, queryKey: queryOptions.queryKey };
}




/**
 * @summary Create a new group
 */
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

export const createGroup = async ( options?: RequestInit): Promise<createGroupResponse> => {

  return customFetch<createGroupResponse>(getCreateGroupUrl(),
  {
    ...options,
    method: 'POST'


  }
);}




export const getCreateGroupMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof createGroup>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): UseMutationOptions<Awaited<ReturnType<typeof createGroup>>, TError,void, TContext> => {

const mutationKey = ['createGroup'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof createGroup>>, void> = () => {


          return  createGroup(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type CreateGroupMutationResult = NonNullable<Awaited<ReturnType<typeof createGroup>>>

    export type CreateGroupMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Create a new group
 */
export const useCreateGroup = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof createGroup>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient): UseMutationResult<
        Awaited<ReturnType<typeof createGroup>>,
        TError,
        void,
        TContext
      > => {
      return useMutation(getCreateGroupMutationOptions(options), queryClient);
    }
    /**
 * @summary Get a specific group
 */
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

export const getGroup = async (id: string, options?: RequestInit): Promise<getGroupResponse> => {

  return customFetch<getGroupResponse>(getGetGroupUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}





export const getGetGroupQueryKey = (id: string,) => {
    return [
    `/api/v1/groups/${id}`
    ] as const;
    }


export const getGetGroupQueryOptions = <TData = Awaited<ReturnType<typeof getGroup>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getGetGroupQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof getGroup>>> = ({ signal }) => getGroup(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: !!(id), ...queryOptions} as UseQueryOptions<Awaited<ReturnType<typeof getGroup>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type GetGroupQueryResult = NonNullable<Awaited<ReturnType<typeof getGroup>>>
export type GetGroupQueryError = ErrorType<ProblemSchema>


export function useGetGroup<TData = Awaited<ReturnType<typeof getGroup>>, TError = ErrorType<ProblemSchema>>(
 id: string, options: { query:Partial<UseQueryOptions<Awaited<ReturnType<typeof getGroup>>, TError, TData>> & Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof getGroup>>,
          TError,
          Awaited<ReturnType<typeof getGroup>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useGetGroup<TData = Awaited<ReturnType<typeof getGroup>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getGroup>>, TError, TData>> & Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof getGroup>>,
          TError,
          Awaited<ReturnType<typeof getGroup>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useGetGroup<TData = Awaited<ReturnType<typeof getGroup>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
/**
 * @summary Get a specific group
 */

export function useGetGroup<TData = Awaited<ReturnType<typeof getGroup>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getGroup>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
 ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {

  const queryOptions = getGetGroupQueryOptions(id,options)

  const query = useQuery(queryOptions, queryClient) as  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return { ...query, queryKey: queryOptions.queryKey };
}




/**
 * @summary Delete a group
 */
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

export const deleteGroup = async (id: string, options?: RequestInit): Promise<deleteGroupResponse> => {

  return customFetch<deleteGroupResponse>(getDeleteGroupUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}




export const getDeleteGroupMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof deleteGroup>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): UseMutationOptions<Awaited<ReturnType<typeof deleteGroup>>, TError,{id: string}, TContext> => {

const mutationKey = ['deleteGroup'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof deleteGroup>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  deleteGroup(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type DeleteGroupMutationResult = NonNullable<Awaited<ReturnType<typeof deleteGroup>>>

    export type DeleteGroupMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Delete a group
 */
export const useDeleteGroup = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof deleteGroup>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient): UseMutationResult<
        Awaited<ReturnType<typeof deleteGroup>>,
        TError,
        {id: string},
        TContext
      > => {
      return useMutation(getDeleteGroupMutationOptions(options), queryClient);
    }
    /**
 * @summary Update a specific group
 */
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

export const updateGroup = async (id: string, options?: RequestInit): Promise<updateGroupResponse> => {

  return customFetch<updateGroupResponse>(getUpdateGroupUrl(id),
  {
    ...options,
    method: 'PATCH'


  }
);}




export const getUpdateGroupMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof updateGroup>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): UseMutationOptions<Awaited<ReturnType<typeof updateGroup>>, TError,{id: string}, TContext> => {

const mutationKey = ['updateGroup'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof updateGroup>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  updateGroup(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type UpdateGroupMutationResult = NonNullable<Awaited<ReturnType<typeof updateGroup>>>

    export type UpdateGroupMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Update a specific group
 */
export const useUpdateGroup = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof updateGroup>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient): UseMutationResult<
        Awaited<ReturnType<typeof updateGroup>>,
        TError,
        {id: string},
        TContext
      > => {
      return useMutation(getUpdateGroupMutationOptions(options), queryClient);
    }
