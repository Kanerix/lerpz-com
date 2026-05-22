// @ts-nocheck
import {
  useQuery
} from '@tanstack/react-query';
import type {
  DataTag,
  DefinedInitialDataOptions,
  DefinedUseQueryResult,
  QueryClient,
  QueryFunction,
  QueryKey,
  UndefinedInitialDataOptions,
  UseQueryOptions,
  UseQueryResult
} from '@tanstack/react-query';

import type {
  HealthCheck,
  ProblemSchema
} from '../models';

import { customFetch } from '../../../lib/orval-mutator';
import type { ErrorType } from '../../../lib/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



/**
 * This is only inlcuded in debug builds
 * @summary Force an API failure
 */
export type triggerFailureResponse500 = {
  data: ProblemSchema
  status: 500
}

;
export type triggerFailureResponseError = (triggerFailureResponse500) & {
  headers: Headers;
};

export type triggerFailureResponse = (triggerFailureResponseError)

export const getTriggerFailureUrl = () => {




  return `/api/v1/failure`
}

export const triggerFailure = async ( options?: RequestInit): Promise<triggerFailureResponse> => {

  return customFetch<triggerFailureResponse>(getTriggerFailureUrl(),
  {
    ...options,
    method: 'GET'


  }
);}





export const getTriggerFailureQueryKey = () => {
    return [
    `/api/v1/failure`
    ] as const;
    }


export const getTriggerFailureQueryOptions = <TData = Awaited<ReturnType<typeof triggerFailure>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof triggerFailure>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getTriggerFailureQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof triggerFailure>>> = ({ signal }) => triggerFailure({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as UseQueryOptions<Awaited<ReturnType<typeof triggerFailure>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type TriggerFailureQueryResult = NonNullable<Awaited<ReturnType<typeof triggerFailure>>>
export type TriggerFailureQueryError = ErrorType<ProblemSchema>


export function useTriggerFailure<TData = Awaited<ReturnType<typeof triggerFailure>>, TError = ErrorType<ProblemSchema>>(
  options: { query:Partial<UseQueryOptions<Awaited<ReturnType<typeof triggerFailure>>, TError, TData>> & Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof triggerFailure>>,
          TError,
          Awaited<ReturnType<typeof triggerFailure>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useTriggerFailure<TData = Awaited<ReturnType<typeof triggerFailure>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof triggerFailure>>, TError, TData>> & Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof triggerFailure>>,
          TError,
          Awaited<ReturnType<typeof triggerFailure>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useTriggerFailure<TData = Awaited<ReturnType<typeof triggerFailure>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof triggerFailure>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
/**
 * @summary Force an API failure
 */

export function useTriggerFailure<TData = Awaited<ReturnType<typeof triggerFailure>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof triggerFailure>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
 ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {

  const queryOptions = getTriggerFailureQueryOptions(options)

  const query = useQuery(queryOptions, queryClient) as  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return { ...query, queryKey: queryOptions.queryKey };
}




/**
 * Verifies connectivity, always returns 200 if health check succeeds.
 * @summary Get API health status
 */
export type healthCheckResponse200 = {
  data: HealthCheck
  status: 200
}

export type healthCheckResponse401 = {
  data: ProblemSchema
  status: 401
}

export type healthCheckResponse500 = {
  data: ProblemSchema
  status: 500
}

export type healthCheckResponseSuccess = (healthCheckResponse200) & {
  headers: Headers;
};
export type healthCheckResponseError = (healthCheckResponse401 | healthCheckResponse500) & {
  headers: Headers;
};

export type healthCheckResponse = (healthCheckResponseSuccess | healthCheckResponseError)

export const getHealthCheckUrl = () => {




  return `/api/v1/health`
}

export const healthCheck = async ( options?: RequestInit): Promise<healthCheckResponse> => {

  return customFetch<healthCheckResponse>(getHealthCheckUrl(),
  {
    ...options,
    method: 'GET'


  }
);}





export const getHealthCheckQueryKey = () => {
    return [
    `/api/v1/health`
    ] as const;
    }


export const getHealthCheckQueryOptions = <TData = Awaited<ReturnType<typeof healthCheck>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof healthCheck>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getHealthCheckQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof healthCheck>>> = ({ signal }) => healthCheck({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as UseQueryOptions<Awaited<ReturnType<typeof healthCheck>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type HealthCheckQueryResult = NonNullable<Awaited<ReturnType<typeof healthCheck>>>
export type HealthCheckQueryError = ErrorType<ProblemSchema>


export function useHealthCheck<TData = Awaited<ReturnType<typeof healthCheck>>, TError = ErrorType<ProblemSchema>>(
  options: { query:Partial<UseQueryOptions<Awaited<ReturnType<typeof healthCheck>>, TError, TData>> & Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof healthCheck>>,
          TError,
          Awaited<ReturnType<typeof healthCheck>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useHealthCheck<TData = Awaited<ReturnType<typeof healthCheck>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof healthCheck>>, TError, TData>> & Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof healthCheck>>,
          TError,
          Awaited<ReturnType<typeof healthCheck>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useHealthCheck<TData = Awaited<ReturnType<typeof healthCheck>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof healthCheck>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
/**
 * @summary Get API health status
 */

export function useHealthCheck<TData = Awaited<ReturnType<typeof healthCheck>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof healthCheck>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
 ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {

  const queryOptions = getHealthCheckQueryOptions(options)

  const query = useQuery(queryOptions, queryClient) as  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return { ...query, queryKey: queryOptions.queryKey };
}




