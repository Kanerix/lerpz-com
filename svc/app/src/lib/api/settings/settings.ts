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
  ProblemSchema,
  UpdateSettingsRequest,
  UserSettings
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type getSettingsResponse200 = {
  data: UserSettings
  status: 200
}

export type getSettingsResponse401 = {
  data: ProblemSchema
  status: 401
}

export type getSettingsResponse500 = {
  data: ProblemSchema
  status: 500
}

export type getSettingsResponseSuccess = (getSettingsResponse200) & {
  headers: Headers;
};
export type getSettingsResponseError = (getSettingsResponse401 | getSettingsResponse500) & {
  headers: Headers;
};

export type getSettingsResponse = (getSettingsResponseSuccess | getSettingsResponseError)

export const getGetSettingsUrl = () => {




  return `/api/v1/settings`
}

/**
 * Returns the authenticated user's account settings. When the user has never saved any settings, the server-side defaults are returned instead.
 * @summary Get account settings
 */
export const getSettings = async ( options?: RequestInit): Promise<getSettingsResponse> => {

  return customFetch<getSettingsResponse>(getGetSettingsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}




export const getGetSettingsMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getSettings>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof getSettings>>, TError,void, TContext> => {

const mutationKey = ['getSettings'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof getSettings>>, void> = () => {


          return  getSettings(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type GetSettingsMutationResult = NonNullable<Awaited<ReturnType<typeof getSettings>>>

    export type GetSettingsMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get account settings
 */
export const createGetSettings = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getSettings>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof getSettings>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getGetSettingsMutationOptions(options?.()) }), queryClient);
    }
    export type updateSettingsResponse200 = {
  data: UserSettings
  status: 200
}

export type updateSettingsResponse400 = {
  data: ProblemSchema
  status: 400
}

export type updateSettingsResponse401 = {
  data: ProblemSchema
  status: 401
}

export type updateSettingsResponse500 = {
  data: ProblemSchema
  status: 500
}

export type updateSettingsResponseSuccess = (updateSettingsResponse200) & {
  headers: Headers;
};
export type updateSettingsResponseError = (updateSettingsResponse400 | updateSettingsResponse401 | updateSettingsResponse500) & {
  headers: Headers;
};

export type updateSettingsResponse = (updateSettingsResponseSuccess | updateSettingsResponseError)

export const getUpdateSettingsUrl = () => {




  return `/api/v1/settings`
}

/**
 * Updates the authenticated user's account settings, creating them on first use. Only the fields present in the request body are changed; omitted fields keep their current value. Returns the full, updated settings.
 * @summary Update account settings
 */
export const updateSettings = async (updateSettingsRequest: UpdateSettingsRequest, options?: RequestInit): Promise<updateSettingsResponse> => {

  return customFetch<updateSettingsResponse>(getUpdateSettingsUrl(),
  {
    ...options,
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(updateSettingsRequest)
  }
);}





export const getUpdateSettingsQueryKey = (updateSettingsRequest?: UpdateSettingsRequest,) => {
    return [
    'PATCH', `/api/v1/settings`, updateSettingsRequest
    ] as const;
    }


export const getUpdateSettingsQueryOptions = <TData = Awaited<ReturnType<typeof updateSettings>>, TError = ErrorType<ProblemSchema>>(updateSettingsRequest: UpdateSettingsRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateSettings>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getUpdateSettingsQueryKey(updateSettingsRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof updateSettings>>> = ({ signal }) => updateSettings(updateSettingsRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof updateSettings>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type UpdateSettingsQueryResult = NonNullable<Awaited<ReturnType<typeof updateSettings>>>
export type UpdateSettingsQueryError = ErrorType<ProblemSchema>


/**
 * @summary Update account settings
 */

export function createUpdateSettings<TData = Awaited<ReturnType<typeof updateSettings>>, TError = ErrorType<ProblemSchema>>(
 updateSettingsRequest: () =>  UpdateSettingsRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateSettings>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getUpdateSettingsQueryOptions(updateSettingsRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






