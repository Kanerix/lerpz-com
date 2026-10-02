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
  CreateVideoResponse,
  ListVideosParams,
  ProblemSchema,
  VideoJobResponse,
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





export const getListVideosQueryKey = (params?: ListVideosParams,) => {
    return [
    `/api/v1/videos`, ...(params ? [params] : [])
    ] as const;
    }


export const getListVideosQueryOptions = <TData = Awaited<ReturnType<typeof listVideos>>, TError = ErrorType<ProblemSchema>>(params?: ListVideosParams, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof listVideos>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getListVideosQueryKey(params);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof listVideos>>> = ({ signal }) => listVideos(params, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof listVideos>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type ListVideosQueryResult = NonNullable<Awaited<ReturnType<typeof listVideos>>>
export type ListVideosQueryError = ErrorType<ProblemSchema>


/**
 * @summary List generated videos
 */

export function createListVideos<TData = Awaited<ReturnType<typeof listVideos>>, TError = ErrorType<ProblemSchema>>(
 params?: () =>  ListVideosParams, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof listVideos>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getListVideosQueryOptions(params?.(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type createVideoResponse202 = {
  data: CreateVideoResponse
  status: 202
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

export type createVideoResponse502 = {
  data: ProblemSchema
  status: 502
}

export type createVideoResponseSuccess = (createVideoResponse202) & {
  headers: Headers;
};
export type createVideoResponseError = (createVideoResponse400 | createVideoResponse401 | createVideoResponse500 | createVideoResponse502) & {
  headers: Headers;
};

export type createVideoResponse = (createVideoResponseSuccess | createVideoResponseError)

export const getCreateVideoUrl = () => {




  return `/api/v1/videos`
}

/**
 * Starts a video generation job and returns immediately with a job id. The render runs in the background; poll `GET /videos/jobs/{id}` until the job reaches a terminal state (`completed` or `failed`).
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




export const getCreateVideoMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof createVideo>>, TError,{data: VideoRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof createVideo>>, TError,{data: VideoRequest}, TContext> => {

const mutationKey = ['createVideo'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof createVideo>>, {data: VideoRequest}> = (props) => {
          const {data} = props ?? {};

          return  createVideo(data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type CreateVideoMutationResult = NonNullable<Awaited<ReturnType<typeof createVideo>>>
    export type CreateVideoMutationBody = VideoRequest
    export type CreateVideoMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Create a new video
 */
export const createCreateVideo = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof createVideo>>, TError,{data: VideoRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof createVideo>>,
        TError,
        {data: VideoRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getCreateVideoMutationOptions(options?.()) }), queryClient);
    }
    export type getVideoJobResponse200 = {
  data: VideoJobResponse
  status: 200
}

export type getVideoJobResponse401 = {
  data: ProblemSchema
  status: 401
}

export type getVideoJobResponse404 = {
  data: ProblemSchema
  status: 404
}

export type getVideoJobResponse500 = {
  data: ProblemSchema
  status: 500
}

export type getVideoJobResponseSuccess = (getVideoJobResponse200) & {
  headers: Headers;
};
export type getVideoJobResponseError = (getVideoJobResponse401 | getVideoJobResponse404 | getVideoJobResponse500) & {
  headers: Headers;
};

export type getVideoJobResponse = (getVideoJobResponseSuccess | getVideoJobResponseError)

export const getGetVideoJobUrl = (id: string,) => {




  return `/api/v1/videos/jobs/${id}`
}

/**
 * Returns the current status of a video generation job. Poll this endpoint after creating a video until `status` is `completed` (the `video` field carries the result) or `failed` (the `error` field carries the reason).
 * @summary Get video job status
 */
export const getVideoJob = async (id: string, options?: RequestInit): Promise<getVideoJobResponse> => {

  return customFetch<getVideoJobResponse>(getGetVideoJobUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}





export const getGetVideoJobQueryKey = (id: string,) => {
    return [
    `/api/v1/videos/jobs/${id}`
    ] as const;
    }


export const getGetVideoJobQueryOptions = <TData = Awaited<ReturnType<typeof getVideoJob>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof getVideoJob>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getGetVideoJobQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof getVideoJob>>> = ({ signal }) => getVideoJob(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof getVideoJob>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type GetVideoJobQueryResult = NonNullable<Awaited<ReturnType<typeof getVideoJob>>>
export type GetVideoJobQueryError = ErrorType<ProblemSchema>


/**
 * @summary Get video job status
 */

export function createGetVideoJob<TData = Awaited<ReturnType<typeof getVideoJob>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof getVideoJob>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getGetVideoJobQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}
