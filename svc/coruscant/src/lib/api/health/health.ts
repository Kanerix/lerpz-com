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
  HealthCheck,
  ProblemSchema
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



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

/**
 * This is only inlcuded in debug builds
 * @summary Force an API failure
 */
export const triggerFailure = async ( options?: RequestInit): Promise<triggerFailureResponse> => {

  return customFetch<triggerFailureResponse>(getTriggerFailureUrl(),
  {
    ...options,
    method: 'GET'


  }
);}




export const getTriggerFailureMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof triggerFailure>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof triggerFailure>>, TError,void, TContext> => {

const mutationKey = ['triggerFailure'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof triggerFailure>>, void> = () => {


          return  triggerFailure(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type TriggerFailureMutationResult = NonNullable<Awaited<ReturnType<typeof triggerFailure>>>

    export type TriggerFailureMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Force an API failure
 */
export const createTriggerFailure = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof triggerFailure>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof triggerFailure>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getTriggerFailureMutationOptions(options?.()) }), queryClient);
    }
    export type healthCheckResponse200 = {
  data: HealthCheck
  status: 200
}

export type healthCheckResponse500 = {
  data: ProblemSchema
  status: 500
}

export type healthCheckResponseSuccess = (healthCheckResponse200) & {
  headers: Headers;
};
export type healthCheckResponseError = (healthCheckResponse500) & {
  headers: Headers;
};

export type healthCheckResponse = (healthCheckResponseSuccess | healthCheckResponseError)

export const getHealthCheckUrl = () => {




  return `/api/v1/health`
}

/**
 * Verifies connectivity, always returns 200 if health check succeeds.
 * @summary Get API health status
 */
export const healthCheck = async ( options?: RequestInit): Promise<healthCheckResponse> => {

  return customFetch<healthCheckResponse>(getHealthCheckUrl(),
  {
    ...options,
    method: 'GET'


  }
);}




export const getHealthCheckMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof healthCheck>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof healthCheck>>, TError,void, TContext> => {

const mutationKey = ['healthCheck'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof healthCheck>>, void> = () => {


          return  healthCheck(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type HealthCheckMutationResult = NonNullable<Awaited<ReturnType<typeof healthCheck>>>

    export type HealthCheckMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get API health status
 */
export const createHealthCheck = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof healthCheck>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof healthCheck>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getHealthCheckMutationOptions(options?.()) }), queryClient);
    }
