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
  ListVideosParams,
  ProblemSchema,
  VideoAnalysisResponse,
  VideoListResponse,
  VideoRequest
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type listVideosResponse200 = {
  data: VideoListResponse
  status: 200
}

export type listVideosResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listVideosResponse500 = {
  data: ProblemSchema
  status: 500
}

export type listVideosResponseSuccess = (listVideosResponse200) & {
  headers: Headers;
};
export type listVideosResponseError = (listVideosResponse401 | listVideosResponse500) & {
  headers: Headers;
};

export type listVideosResponse = (listVideosResponseSuccess | listVideosResponseError)

export const getListVideosUrl = (params?: ListVideosParams,) => {
  const normalizedParams = new URLSearchParams();

  Object.entries(params || {}).forEach(([key, value]) => {

    if (value !== undefined) {
      normalizedParams.append(key, value === null ? 'null' : String(value))
    }
  });

  const stringifiedParams = normalizedParams.toString();

  return stringifiedParams.length > 0 ? `/api/v1/videos?${stringifiedParams}` : `/api/v1/videos`
}

/**
 * Returns a page of generated videos, newest first, using simple cursor-based pagination. Each item carries a public `url` served from the storage bucket (acting as a CDN). Pass the returned `next_cursor` back as the `cursor` query parameter to load the next page; a `null` cursor means there are no more videos.
 * @summary List generated videos
 */
export const listVideos = async (params?: ListVideosParams, options?: RequestInit): Promise<listVideosResponse> => {

  return customFetch<listVideosResponse>(getListVideosUrl(params),
  {
    ...options,
    method: 'GET'


  }
);}




export const getListVideosMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listVideos>>, TError,{params?: ListVideosParams}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof listVideos>>, TError,{params?: ListVideosParams}, TContext> => {

const mutationKey = ['listVideos'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof listVideos>>, {params?: ListVideosParams}> = (props) => {
          const {params} = props ?? {};

          return  listVideos(params,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type ListVideosMutationResult = NonNullable<Awaited<ReturnType<typeof listVideos>>>

    export type ListVideosMutationError = ErrorType<ProblemSchema>

    /**
 * @summary List generated videos
 */
export const createListVideos = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listVideos>>, TError,{params?: ListVideosParams}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof listVideos>>,
        TError,
        {params?: ListVideosParams},
        TContext
      > => {
      return createMutation(() => ({ ...getListVideosMutationOptions(options?.()) }), queryClient);
    }
    export type createVideoResponse400 = {
  data: ProblemSchema
  status: 400
}

export type createVideoResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createVideoResponse500 = {
  data: ProblemSchema
  status: 500
}

;
export type createVideoResponseError = (createVideoResponse400 | createVideoResponse401 | createVideoResponse500) & {
  headers: Headers;
};

export type createVideoResponse = (createVideoResponseError)

export const getCreateVideoUrl = () => {




  return `/api/v1/videos`
}

/**
 * @summary Create a new video
 */
export const createVideo = async (videoRequest: VideoRequest, options?: RequestInit): Promise<createVideoResponse> => {

  return customFetch<createVideoResponse>(getCreateVideoUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(videoRequest)
  }
);}





export const getCreateVideoQueryKey = (videoRequest?: VideoRequest,) => {
    return [
    'POST', `/api/v1/videos`, videoRequest
    ] as const;
    }


export const getCreateVideoQueryOptions = <TData = Awaited<ReturnType<typeof createVideo>>, TError = ErrorType<ProblemSchema>>(videoRequest: VideoRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createVideo>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getCreateVideoQueryKey(videoRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof createVideo>>> = ({ signal }) => createVideo(videoRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof createVideo>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type CreateVideoQueryResult = NonNullable<Awaited<ReturnType<typeof createVideo>>>
export type CreateVideoQueryError = ErrorType<ProblemSchema>


/**
 * @summary Create a new video
 */

export function createCreateVideo<TData = Awaited<ReturnType<typeof createVideo>>, TError = ErrorType<ProblemSchema>>(
 videoRequest: () =>  VideoRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createVideo>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getCreateVideoQueryOptions(videoRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type analyzeVideoResponse200 = {
  data: VideoAnalysisResponse
  status: 200
}

export type analyzeVideoResponse401 = {
  data: ProblemSchema
  status: 401
}

export type analyzeVideoResponse404 = {
  data: ProblemSchema
  status: 404
}

export type analyzeVideoResponse500 = {
  data: ProblemSchema
  status: 500
}

export type analyzeVideoResponseSuccess = (analyzeVideoResponse200) & {
  headers: Headers;
};
export type analyzeVideoResponseError = (analyzeVideoResponse401 | analyzeVideoResponse404 | analyzeVideoResponse500) & {
  headers: Headers;
};

export type analyzeVideoResponse = (analyzeVideoResponseSuccess | analyzeVideoResponseError)

export const getAnalyzeVideoUrl = (id: string,) => {




  return `/api/v1/videos/${id}/analysis`
}

/**
 * Runs a vision model over a previously generated video to produce a descriptive title and a set of tags, persists them to the video's metadata, and returns them. Re-running the analysis overwrites any existing title and tags.
 * @summary Analyse a video
 */
export const analyzeVideo = async (id: string, options?: RequestInit): Promise<analyzeVideoResponse> => {

  return customFetch<analyzeVideoResponse>(getAnalyzeVideoUrl(id),
  {
    ...options,
    method: 'POST'


  }
);}





export const getAnalyzeVideoQueryKey = (id: string,) => {
    return [
    'POST', `/api/v1/videos/${id}/analysis`
    ] as const;
    }


export const getAnalyzeVideoQueryOptions = <TData = Awaited<ReturnType<typeof analyzeVideo>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof analyzeVideo>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getAnalyzeVideoQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof analyzeVideo>>> = ({ signal }) => analyzeVideo(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof analyzeVideo>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type AnalyzeVideoQueryResult = NonNullable<Awaited<ReturnType<typeof analyzeVideo>>>
export type AnalyzeVideoQueryError = ErrorType<ProblemSchema>


/**
 * @summary Analyse a video
 */

export function createAnalyzeVideo<TData = Awaited<ReturnType<typeof analyzeVideo>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof analyzeVideo>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getAnalyzeVideoQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






