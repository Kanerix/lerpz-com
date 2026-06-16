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
  ImageRequest,
  ProblemSchema
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type createImageResponse400 = {
  data: ProblemSchema
  status: 400
}

export type createImageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createImageResponse500 = {
  data: ProblemSchema
  status: 500
}

;
export type createImageResponseError = (createImageResponse400 | createImageResponse401 | createImageResponse500) & {
  headers: Headers;
};

export type createImageResponse = (createImageResponseError)

export const getCreateImageUrl = () => {




  return `/api/v1/images`
}

/**
 * @summary Create a new image
 */
export const createImage = async (imageRequest: ImageRequest, options?: RequestInit): Promise<createImageResponse> => {

  return customFetch<createImageResponse>(getCreateImageUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(imageRequest)
  }
);}





export const getCreateImageQueryKey = (imageRequest?: ImageRequest,) => {
    return [
    'POST', `/api/v1/images`, imageRequest
    ] as const;
    }


export const getCreateImageQueryOptions = <TData = Awaited<ReturnType<typeof createImage>>, TError = ErrorType<ProblemSchema>>(imageRequest: ImageRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createImage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getCreateImageQueryKey(imageRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof createImage>>> = ({ signal }) => createImage(imageRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof createImage>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type CreateImageQueryResult = NonNullable<Awaited<ReturnType<typeof createImage>>>
export type CreateImageQueryError = ErrorType<ProblemSchema>


/**
 * @summary Create a new image
 */

export function createCreateImage<TData = Awaited<ReturnType<typeof createImage>>, TError = ErrorType<ProblemSchema>>(
 imageRequest: () =>  ImageRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createImage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getCreateImageQueryOptions(imageRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type editImageResponse200 = {
  data: void
  status: 200
}

export type editImageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type editImageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type editImageResponseSuccess = (editImageResponse200) & {
  headers: Headers;
};
export type editImageResponseError = (editImageResponse401 | editImageResponse500) & {
  headers: Headers;
};

export type editImageResponse = (editImageResponseSuccess | editImageResponseError)

export const getEditImageUrl = () => {




  return `/api/v1/images/edit`
}

/**
 * @summary Create a new image from existing images
 */
export const editImage = async ( options?: RequestInit): Promise<editImageResponse> => {

  return customFetch<editImageResponse>(getEditImageUrl(),
  {
    ...options,
    method: 'POST'


  }
);}





export const getEditImageQueryKey = () => {
    return [
    'POST', `/api/v1/images/edit`
    ] as const;
    }


export const getEditImageQueryOptions = <TData = Awaited<ReturnType<typeof editImage>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof editImage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getEditImageQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof editImage>>> = ({ signal }) => editImage({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof editImage>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type EditImageQueryResult = NonNullable<Awaited<ReturnType<typeof editImage>>>
export type EditImageQueryError = ErrorType<ProblemSchema>


/**
 * @summary Create a new image from existing images
 */

export function createEditImage<TData = Awaited<ReturnType<typeof editImage>>, TError = ErrorType<ProblemSchema>>(
  options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof editImage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getEditImageQueryOptions(options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type deleteImageResponse200 = {
  data: void
  status: 200
}

export type deleteImageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteImageResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteImageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteImageResponseSuccess = (deleteImageResponse200) & {
  headers: Headers;
};
export type deleteImageResponseError = (deleteImageResponse401 | deleteImageResponse404 | deleteImageResponse500) & {
  headers: Headers;
};

export type deleteImageResponse = (deleteImageResponseSuccess | deleteImageResponseError)

export const getDeleteImageUrl = (id: string,) => {




  return `/api/v1/images/${id}`
}

/**
 * @summary Delete a specific image
 */
export const deleteImage = async (id: string, options?: RequestInit): Promise<deleteImageResponse> => {

  return customFetch<deleteImageResponse>(getDeleteImageUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}





export const getDeleteImageQueryKey = (id: string,) => {
    return [
    'DELETE', `/api/v1/images/${id}`
    ] as const;
    }


export const getDeleteImageQueryOptions = <TData = Awaited<ReturnType<typeof deleteImage>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteImage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getDeleteImageQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof deleteImage>>> = ({ signal }) => deleteImage(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof deleteImage>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type DeleteImageQueryResult = NonNullable<Awaited<ReturnType<typeof deleteImage>>>
export type DeleteImageQueryError = ErrorType<ProblemSchema>


/**
 * @summary Delete a specific image
 */

export function createDeleteImage<TData = Awaited<ReturnType<typeof deleteImage>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteImage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getDeleteImageQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






