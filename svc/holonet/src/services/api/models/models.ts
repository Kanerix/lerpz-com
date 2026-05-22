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
  Models,
  ProblemSchema
} from './.';

import { customFetch } from '../../../lib/orval-mutator';
import type { ErrorType } from '../../../lib/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



/**
 * Returns the list of AI models available to the authenticated user. Each model includes a human-readable name, a namespaced slug used when making inference requests, and a provider family identifier (e.g. `openai`, `google-ai`).
 * @summary Get available models
 */
export type listModelsResponse200 = {
  data: Models
  status: 200
}

export type listModelsResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listModelsResponse500 = {
  data: ProblemSchema
  status: 500
}

export type listModelsResponseSuccess = (listModelsResponse200) & {
  headers: Headers;
};
export type listModelsResponseError = (listModelsResponse401 | listModelsResponse500) & {
  headers: Headers;
};

export type listModelsResponse = (listModelsResponseSuccess | listModelsResponseError)

export const getListModelsUrl = () => {




  return `/api/v1/models`
}

export const listModels = async ( options?: RequestInit): Promise<listModelsResponse> => {

  return customFetch<listModelsResponse>(getListModelsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}





export const getListModelsQueryKey = () => {
    return [
    `/api/v1/models`
    ] as const;
    }


export const getListModelsQueryOptions = <TData = Awaited<ReturnType<typeof listModels>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listModels>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getListModelsQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof listModels>>> = ({ signal }) => listModels({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as UseQueryOptions<Awaited<ReturnType<typeof listModels>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type ListModelsQueryResult = NonNullable<Awaited<ReturnType<typeof listModels>>>
export type ListModelsQueryError = ErrorType<ProblemSchema>


export function useListModels<TData = Awaited<ReturnType<typeof listModels>>, TError = ErrorType<ProblemSchema>>(
  options: { query:Partial<UseQueryOptions<Awaited<ReturnType<typeof listModels>>, TError, TData>> & Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof listModels>>,
          TError,
          Awaited<ReturnType<typeof listModels>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useListModels<TData = Awaited<ReturnType<typeof listModels>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listModels>>, TError, TData>> & Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof listModels>>,
          TError,
          Awaited<ReturnType<typeof listModels>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useListModels<TData = Awaited<ReturnType<typeof listModels>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listModels>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
/**
 * @summary Get available models
 */

export function useListModels<TData = Awaited<ReturnType<typeof listModels>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listModels>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
 ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {

  const queryOptions = getListModelsQueryOptions(options)

  const query = useQuery(queryOptions, queryClient) as  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return { ...query, queryKey: queryOptions.queryKey };
}




/**
 * @summary Create a new model
 */
export type createModelResponse200 = {
  data: void
  status: 200
}

export type createModelResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createModelResponse500 = {
  data: ProblemSchema
  status: 500
}

export type createModelResponseSuccess = (createModelResponse200) & {
  headers: Headers;
};
export type createModelResponseError = (createModelResponse401 | createModelResponse500) & {
  headers: Headers;
};

export type createModelResponse = (createModelResponseSuccess | createModelResponseError)

export const getCreateModelUrl = () => {




  return `/api/v1/models`
}

export const createModel = async ( options?: RequestInit): Promise<createModelResponse> => {

  return customFetch<createModelResponse>(getCreateModelUrl(),
  {
    ...options,
    method: 'POST'


  }
);}




export const getCreateModelMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof createModel>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): UseMutationOptions<Awaited<ReturnType<typeof createModel>>, TError,void, TContext> => {

const mutationKey = ['createModel'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof createModel>>, void> = () => {


          return  createModel(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type CreateModelMutationResult = NonNullable<Awaited<ReturnType<typeof createModel>>>

    export type CreateModelMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Create a new model
 */
export const useCreateModel = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof createModel>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient): UseMutationResult<
        Awaited<ReturnType<typeof createModel>>,
        TError,
        void,
        TContext
      > => {
      return useMutation(getCreateModelMutationOptions(options), queryClient);
    }
    /**
 * @summary Get a specific model
 */
export type getModelResponse200 = {
  data: void
  status: 200
}

export type getModelResponse401 = {
  data: ProblemSchema
  status: 401
}

export type getModelResponse404 = {
  data: ProblemSchema
  status: 404
}

export type getModelResponse500 = {
  data: ProblemSchema
  status: 500
}

export type getModelResponseSuccess = (getModelResponse200) & {
  headers: Headers;
};
export type getModelResponseError = (getModelResponse401 | getModelResponse404 | getModelResponse500) & {
  headers: Headers;
};

export type getModelResponse = (getModelResponseSuccess | getModelResponseError)

export const getGetModelUrl = (id: string,) => {




  return `/api/v1/models/${id}`
}

export const getModel = async (id: string, options?: RequestInit): Promise<getModelResponse> => {

  return customFetch<getModelResponse>(getGetModelUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}





export const getGetModelQueryKey = (id: string,) => {
    return [
    `/api/v1/models/${id}`
    ] as const;
    }


export const getGetModelQueryOptions = <TData = Awaited<ReturnType<typeof getModel>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getGetModelQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof getModel>>> = ({ signal }) => getModel(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: !!(id), ...queryOptions} as UseQueryOptions<Awaited<ReturnType<typeof getModel>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type GetModelQueryResult = NonNullable<Awaited<ReturnType<typeof getModel>>>
export type GetModelQueryError = ErrorType<ProblemSchema>


export function useGetModel<TData = Awaited<ReturnType<typeof getModel>>, TError = ErrorType<ProblemSchema>>(
 id: string, options: { query:Partial<UseQueryOptions<Awaited<ReturnType<typeof getModel>>, TError, TData>> & Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof getModel>>,
          TError,
          Awaited<ReturnType<typeof getModel>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useGetModel<TData = Awaited<ReturnType<typeof getModel>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getModel>>, TError, TData>> & Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof getModel>>,
          TError,
          Awaited<ReturnType<typeof getModel>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useGetModel<TData = Awaited<ReturnType<typeof getModel>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
/**
 * @summary Get a specific model
 */

export function useGetModel<TData = Awaited<ReturnType<typeof getModel>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
 ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {

  const queryOptions = getGetModelQueryOptions(id,options)

  const query = useQuery(queryOptions, queryClient) as  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return { ...query, queryKey: queryOptions.queryKey };
}




/**
 * @summary Delete a model
 */
export type deleteModelResponse200 = {
  data: void
  status: 200
}

export type deleteModelResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteModelResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteModelResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteModelResponseSuccess = (deleteModelResponse200) & {
  headers: Headers;
};
export type deleteModelResponseError = (deleteModelResponse401 | deleteModelResponse404 | deleteModelResponse500) & {
  headers: Headers;
};

export type deleteModelResponse = (deleteModelResponseSuccess | deleteModelResponseError)

export const getDeleteModelUrl = (id: string,) => {




  return `/api/v1/models/${id}`
}

export const deleteModel = async (id: string, options?: RequestInit): Promise<deleteModelResponse> => {

  return customFetch<deleteModelResponse>(getDeleteModelUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}




export const getDeleteModelMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof deleteModel>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): UseMutationOptions<Awaited<ReturnType<typeof deleteModel>>, TError,{id: string}, TContext> => {

const mutationKey = ['deleteModel'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof deleteModel>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  deleteModel(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type DeleteModelMutationResult = NonNullable<Awaited<ReturnType<typeof deleteModel>>>

    export type DeleteModelMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Delete a model
 */
export const useDeleteModel = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof deleteModel>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient): UseMutationResult<
        Awaited<ReturnType<typeof deleteModel>>,
        TError,
        {id: string},
        TContext
      > => {
      return useMutation(getDeleteModelMutationOptions(options), queryClient);
    }
    /**
 * @summary Update a specific model
 */
export type updateModelResponse200 = {
  data: void
  status: 200
}

export type updateModelResponse401 = {
  data: ProblemSchema
  status: 401
}

export type updateModelResponse404 = {
  data: ProblemSchema
  status: 404
}

export type updateModelResponse500 = {
  data: ProblemSchema
  status: 500
}

export type updateModelResponseSuccess = (updateModelResponse200) & {
  headers: Headers;
};
export type updateModelResponseError = (updateModelResponse401 | updateModelResponse404 | updateModelResponse500) & {
  headers: Headers;
};

export type updateModelResponse = (updateModelResponseSuccess | updateModelResponseError)

export const getUpdateModelUrl = (id: string,) => {




  return `/api/v1/models/${id}`
}

export const updateModel = async (id: string, options?: RequestInit): Promise<updateModelResponse> => {

  return customFetch<updateModelResponse>(getUpdateModelUrl(id),
  {
    ...options,
    method: 'PATCH'


  }
);}




export const getUpdateModelMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof updateModel>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): UseMutationOptions<Awaited<ReturnType<typeof updateModel>>, TError,{id: string}, TContext> => {

const mutationKey = ['updateModel'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof updateModel>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  updateModel(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type UpdateModelMutationResult = NonNullable<Awaited<ReturnType<typeof updateModel>>>

    export type UpdateModelMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Update a specific model
 */
export const useUpdateModel = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof updateModel>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient): UseMutationResult<
        Awaited<ReturnType<typeof updateModel>>,
        TError,
        {id: string},
        TContext
      > => {
      return useMutation(getUpdateModelMutationOptions(options), queryClient);
    }
