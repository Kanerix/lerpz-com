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
  AnalyzeUploadRequest,
  ImageAnalysisResponse,
  ImageListResponse,
  ImageRequest,
  ListImagesParams,
  ProblemSchema
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type listImagesResponse200 = {
  data: ImageListResponse
  status: 200
}

export type listImagesResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listImagesResponse500 = {
  data: ProblemSchema
  status: 500
}

export type listImagesResponseSuccess = (listImagesResponse200) & {
  headers: Headers;
};
export type listImagesResponseError = (listImagesResponse401 | listImagesResponse500) & {
  headers: Headers;
};

export type listImagesResponse = (listImagesResponseSuccess | listImagesResponseError)

export const getListImagesUrl = (params?: ListImagesParams,) => {
  const normalizedParams = new URLSearchParams();

  Object.entries(params || {}).forEach(([key, value]) => {

    if (value !== undefined) {
      normalizedParams.append(key, value === null ? 'null' : String(value))
    }
  });

  const stringifiedParams = normalizedParams.toString();

  return stringifiedParams.length > 0 ? `/api/v1/images?${stringifiedParams}` : `/api/v1/images`
}

/**
 * Returns a page of generated images, newest first, using simple cursor-based pagination. Each item carries a public `url` served from the storage bucket (acting as a CDN). Pass the returned `next_cursor` back as the `cursor` query parameter to load the next page; a `null` cursor means there are no more images.
 * @summary List generated images
 */
export const listImages = async (params?: ListImagesParams, options?: RequestInit): Promise<listImagesResponse> => {

  return customFetch<listImagesResponse>(getListImagesUrl(params),
  {
    ...options,
    method: 'GET'


  }
);}





export const getListImagesQueryKey = (params?: ListImagesParams,) => {
    return [
    `/api/v1/images`, ...(params ? [params] : [])
    ] as const;
    }


export const getListImagesQueryOptions = <TData = Awaited<ReturnType<typeof listImages>>, TError = ErrorType<ProblemSchema>>(params?: ListImagesParams, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof listImages>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getListImagesQueryKey(params);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof listImages>>> = ({ signal }) => listImages(params, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof listImages>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type ListImagesQueryResult = NonNullable<Awaited<ReturnType<typeof listImages>>>
export type ListImagesQueryError = ErrorType<ProblemSchema>


/**
 * @summary List generated images
 */

export function createListImages<TData = Awaited<ReturnType<typeof listImages>>, TError = ErrorType<ProblemSchema>>(
 params?: () =>  ListImagesParams, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof listImages>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getListImagesQueryOptions(params?.(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






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




export const getCreateImageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof createImage>>, TError,{data: ImageRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof createImage>>, TError,{data: ImageRequest}, TContext> => {

const mutationKey = ['createImage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof createImage>>, {data: ImageRequest}> = (props) => {
          const {data} = props ?? {};

          return  createImage(data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type CreateImageMutationResult = NonNullable<Awaited<ReturnType<typeof createImage>>>
    export type CreateImageMutationBody = ImageRequest
    export type CreateImageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Create a new image
 */
export const createCreateImage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof createImage>>, TError,{data: ImageRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof createImage>>,
        TError,
        {data: ImageRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getCreateImageMutationOptions(options?.()) }), queryClient);
    }
    export type analyzeUploadedImageResponse200 = {
  data: ImageAnalysisResponse
  status: 200
}

export type analyzeUploadedImageResponse400 = {
  data: ProblemSchema
  status: 400
}

export type analyzeUploadedImageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type analyzeUploadedImageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type analyzeUploadedImageResponseSuccess = (analyzeUploadedImageResponse200) & {
  headers: Headers;
};
export type analyzeUploadedImageResponseError = (analyzeUploadedImageResponse400 | analyzeUploadedImageResponse401 | analyzeUploadedImageResponse500) & {
  headers: Headers;
};

export type analyzeUploadedImageResponse = (analyzeUploadedImageResponseSuccess | analyzeUploadedImageResponseError)

export const getAnalyzeUploadedImageUrl = () => {




  return `/api/v1/images/analysis`
}

/**
 * Runs a vision model over a caller-supplied image to produce a descriptive title and a set of tags, and returns them. Unlike analysing a stored image, the upload is not persisted and nothing is written to the database.
 * @summary Analyse an uploaded image
 */
export const analyzeUploadedImage = async (analyzeUploadRequest: AnalyzeUploadRequest, options?: RequestInit): Promise<analyzeUploadedImageResponse> => {

  return customFetch<analyzeUploadedImageResponse>(getAnalyzeUploadedImageUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(analyzeUploadRequest)
  }
);}




export const getAnalyzeUploadedImageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof analyzeUploadedImage>>, TError,{data: AnalyzeUploadRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof analyzeUploadedImage>>, TError,{data: AnalyzeUploadRequest}, TContext> => {

const mutationKey = ['analyzeUploadedImage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof analyzeUploadedImage>>, {data: AnalyzeUploadRequest}> = (props) => {
          const {data} = props ?? {};

          return  analyzeUploadedImage(data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type AnalyzeUploadedImageMutationResult = NonNullable<Awaited<ReturnType<typeof analyzeUploadedImage>>>
    export type AnalyzeUploadedImageMutationBody = AnalyzeUploadRequest
    export type AnalyzeUploadedImageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Analyse an uploaded image
 */
export const createAnalyzeUploadedImage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof analyzeUploadedImage>>, TError,{data: AnalyzeUploadRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof analyzeUploadedImage>>,
        TError,
        {data: AnalyzeUploadRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getAnalyzeUploadedImageMutationOptions(options?.()) }), queryClient);
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




export const getEditImageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof editImage>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof editImage>>, TError,void, TContext> => {

const mutationKey = ['editImage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof editImage>>, void> = () => {


          return  editImage(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type EditImageMutationResult = NonNullable<Awaited<ReturnType<typeof editImage>>>

    export type EditImageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Create a new image from existing images
 */
export const createEditImage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof editImage>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof editImage>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getEditImageMutationOptions(options?.()) }), queryClient);
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




export const getDeleteImageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof deleteImage>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof deleteImage>>, TError,{id: string}, TContext> => {

const mutationKey = ['deleteImage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof deleteImage>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  deleteImage(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type DeleteImageMutationResult = NonNullable<Awaited<ReturnType<typeof deleteImage>>>

    export type DeleteImageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Delete a specific image
 */
export const createDeleteImage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof deleteImage>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof deleteImage>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getDeleteImageMutationOptions(options?.()) }), queryClient);
    }
    export type analyzeImageResponse200 = {
  data: ImageAnalysisResponse
  status: 200
}

export type analyzeImageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type analyzeImageResponse404 = {
  data: ProblemSchema
  status: 404
}

export type analyzeImageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type analyzeImageResponseSuccess = (analyzeImageResponse200) & {
  headers: Headers;
};
export type analyzeImageResponseError = (analyzeImageResponse401 | analyzeImageResponse404 | analyzeImageResponse500) & {
  headers: Headers;
};

export type analyzeImageResponse = (analyzeImageResponseSuccess | analyzeImageResponseError)

export const getAnalyzeImageUrl = (id: string,) => {




  return `/api/v1/images/${id}/analysis`
}

/**
 * Runs a vision model over a previously generated image to produce a descriptive title and a set of tags, persists them to the image's metadata, and returns them. Re-running the analysis overwrites any existing title and tags.
 * @summary Analyse an image
 */
export const analyzeImage = async (id: string, options?: RequestInit): Promise<analyzeImageResponse> => {

  return customFetch<analyzeImageResponse>(getAnalyzeImageUrl(id),
  {
    ...options,
    method: 'POST'


  }
);}




export const getAnalyzeImageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof analyzeImage>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof analyzeImage>>, TError,{id: string}, TContext> => {

const mutationKey = ['analyzeImage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof analyzeImage>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  analyzeImage(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type AnalyzeImageMutationResult = NonNullable<Awaited<ReturnType<typeof analyzeImage>>>

    export type AnalyzeImageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Analyse an image
 */
export const createAnalyzeImage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof analyzeImage>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof analyzeImage>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getAnalyzeImageMutationOptions(options?.()) }), queryClient);
    }
