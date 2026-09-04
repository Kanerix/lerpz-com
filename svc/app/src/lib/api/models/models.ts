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
  CreateModelRequest,
  Model,
  ProblemSchema,
  UpdateModelRequest
} from './.';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type listModelsResponse200 = {
  data: Model[]
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

/**
 * Returns the list of AI models available to the authenticated user, ordered by display name.
 * @summary Get available models
 */
export const listModels = async ( options?: RequestInit): Promise<listModelsResponse> => {

  return customFetch<listModelsResponse>(getListModelsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}




export const getListModelsMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listModels>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof listModels>>, TError,void, TContext> => {

const mutationKey = ['listModels'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof listModels>>, void> = () => {


          return  listModels(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type ListModelsMutationResult = NonNullable<Awaited<ReturnType<typeof listModels>>>

    export type ListModelsMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get available models
 */
export const createListModels = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listModels>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof listModels>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getListModelsMutationOptions(options?.()) }), queryClient);
    }
    export type createModelResponse201 = {
  data: Model
  status: 201
}

export type createModelResponse400 = {
  data: ProblemSchema
  status: 400
}

export type createModelResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createModelResponse409 = {
  data: ProblemSchema
  status: 409
}

export type createModelResponse500 = {
  data: ProblemSchema
  status: 500
}

export type createModelResponseSuccess = (createModelResponse201) & {
  headers: Headers;
};
export type createModelResponseError = (createModelResponse400 | createModelResponse401 | createModelResponse409 | createModelResponse500) & {
  headers: Headers;
};

export type createModelResponse = (createModelResponseSuccess | createModelResponseError)

export const getCreateModelUrl = () => {




  return `/api/v1/models`
}

/**
 * Registers a new model and its Portkey routing configuration.
 * @summary Create a new model
 */
export const createModel = async (createModelRequest: CreateModelRequest, options?: RequestInit): Promise<createModelResponse> => {

  return customFetch<createModelResponse>(getCreateModelUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(createModelRequest)
  }
);}





export const getCreateModelQueryKey = (createModelRequest?: CreateModelRequest,) => {
    return [
    'POST', `/api/v1/models`, createModelRequest
    ] as const;
    }


export const getCreateModelQueryOptions = <TData = Awaited<ReturnType<typeof createModel>>, TError = ErrorType<ProblemSchema>>(createModelRequest: CreateModelRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getCreateModelQueryKey(createModelRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof createModel>>> = ({ signal }) => createModel(createModelRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof createModel>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type CreateModelQueryResult = NonNullable<Awaited<ReturnType<typeof createModel>>>
export type CreateModelQueryError = ErrorType<ProblemSchema>


/**
 * @summary Create a new model
 */

export function createCreateModel<TData = Awaited<ReturnType<typeof createModel>>, TError = ErrorType<ProblemSchema>>(
 createModelRequest: () =>  CreateModelRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getCreateModelQueryOptions(createModelRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type getModelResponse200 = {
  data: Model
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

/**
 * Returns a single model by its identifier.
 * @summary Get a specific model
 */
export const getModel = async (id: string, options?: RequestInit): Promise<getModelResponse> => {

  return customFetch<getModelResponse>(getGetModelUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}




export const getGetModelMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getModel>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof getModel>>, TError,{id: string}, TContext> => {

const mutationKey = ['getModel'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof getModel>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  getModel(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type GetModelMutationResult = NonNullable<Awaited<ReturnType<typeof getModel>>>

    export type GetModelMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get a specific model
 */
export const createGetModel = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getModel>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof getModel>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getGetModelMutationOptions(options?.()) }), queryClient);
    }
    export type deleteModelResponse204 = {
  data: void
  status: 204
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

export type deleteModelResponseSuccess = (deleteModelResponse204) & {
  headers: Headers;
};
export type deleteModelResponseError = (deleteModelResponse401 | deleteModelResponse404 | deleteModelResponse500) & {
  headers: Headers;
};

export type deleteModelResponse = (deleteModelResponseSuccess | deleteModelResponseError)

export const getDeleteModelUrl = (id: string,) => {




  return `/api/v1/models/${id}`
}

/**
 * Permanently deletes a model by its identifier.
 * @summary Delete a model
 */
export const deleteModel = async (id: string, options?: RequestInit): Promise<deleteModelResponse> => {

  return customFetch<deleteModelResponse>(getDeleteModelUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}





export const getDeleteModelQueryKey = (id: string,) => {
    return [
    'DELETE', `/api/v1/models/${id}`
    ] as const;
    }


export const getDeleteModelQueryOptions = <TData = Awaited<ReturnType<typeof deleteModel>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getDeleteModelQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof deleteModel>>> = ({ signal }) => deleteModel(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof deleteModel>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type DeleteModelQueryResult = NonNullable<Awaited<ReturnType<typeof deleteModel>>>
export type DeleteModelQueryError = ErrorType<ProblemSchema>


/**
 * @summary Delete a model
 */

export function createDeleteModel<TData = Awaited<ReturnType<typeof deleteModel>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getDeleteModelQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type updateModelResponse200 = {
  data: Model
  status: 200
}

export type updateModelResponse400 = {
  data: ProblemSchema
  status: 400
}

export type updateModelResponse401 = {
  data: ProblemSchema
  status: 401
}

export type updateModelResponse404 = {
  data: ProblemSchema
  status: 404
}

export type updateModelResponse409 = {
  data: ProblemSchema
  status: 409
}

export type updateModelResponse500 = {
  data: ProblemSchema
  status: 500
}

export type updateModelResponseSuccess = (updateModelResponse200) & {
  headers: Headers;
};
export type updateModelResponseError = (updateModelResponse400 | updateModelResponse401 | updateModelResponse404 | updateModelResponse409 | updateModelResponse500) & {
  headers: Headers;
};

export type updateModelResponse = (updateModelResponseSuccess | updateModelResponseError)

export const getUpdateModelUrl = (id: string,) => {




  return `/api/v1/models/${id}`
}

/**
 * Partially updates a model. Only the provided fields are changed.
 * @summary Update a specific model
 */
export const updateModel = async (id: string,
    updateModelRequest: UpdateModelRequest, options?: RequestInit): Promise<updateModelResponse> => {

  return customFetch<updateModelResponse>(getUpdateModelUrl(id),
  {
    ...options,
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(updateModelRequest)
  }
);}





export const getUpdateModelQueryKey = (id: string,
    updateModelRequest?: UpdateModelRequest,) => {
    return [
    'PATCH', `/api/v1/models/${id}`, updateModelRequest
    ] as const;
    }


export const getUpdateModelQueryOptions = <TData = Awaited<ReturnType<typeof updateModel>>, TError = ErrorType<ProblemSchema>>(id: string,
    updateModelRequest: UpdateModelRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getUpdateModelQueryKey(id,updateModelRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof updateModel>>> = ({ signal }) => updateModel(id,updateModelRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof updateModel>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type UpdateModelQueryResult = NonNullable<Awaited<ReturnType<typeof updateModel>>>
export type UpdateModelQueryError = ErrorType<ProblemSchema>


/**
 * @summary Update a specific model
 */

export function createUpdateModel<TData = Awaited<ReturnType<typeof updateModel>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string,
    updateModelRequest: () =>  UpdateModelRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateModel>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getUpdateModelQueryOptions(id(),
    updateModelRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






