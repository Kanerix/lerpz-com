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
  ChatRequest,
  Conversation,
  ConversationDetail,
  MessageRequest,
  ProblemSchema
} from '../models';

import { customFetch } from '../../../lib/orval-mutator';
import type { ErrorType } from '../../../lib/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



/**
 * Returns a list of the authenticated user's conversations ordered by most recently updated.
 * @summary Get a list of chats
 */
export type listChatsResponse200 = {
  data: Conversation[]
  status: 200
}

export type listChatsResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listChatsResponse500 = {
  data: ProblemSchema
  status: 500
}

export type listChatsResponseSuccess = (listChatsResponse200) & {
  headers: Headers;
};
export type listChatsResponseError = (listChatsResponse401 | listChatsResponse500) & {
  headers: Headers;
};

export type listChatsResponse = (listChatsResponseSuccess | listChatsResponseError)

export const getListChatsUrl = () => {




  return `/api/v1/chats`
}

export const listChats = async ( options?: RequestInit): Promise<listChatsResponse> => {

  return customFetch<listChatsResponse>(getListChatsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}





export const getListChatsQueryKey = () => {
    return [
    `/api/v1/chats`
    ] as const;
    }


export const getListChatsQueryOptions = <TData = Awaited<ReturnType<typeof listChats>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getListChatsQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof listChats>>> = ({ signal }) => listChats({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as UseQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type ListChatsQueryResult = NonNullable<Awaited<ReturnType<typeof listChats>>>
export type ListChatsQueryError = ErrorType<ProblemSchema>


export function useListChats<TData = Awaited<ReturnType<typeof listChats>>, TError = ErrorType<ProblemSchema>>(
  options: { query:Partial<UseQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData>> & Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof listChats>>,
          TError,
          Awaited<ReturnType<typeof listChats>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useListChats<TData = Awaited<ReturnType<typeof listChats>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData>> & Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof listChats>>,
          TError,
          Awaited<ReturnType<typeof listChats>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useListChats<TData = Awaited<ReturnType<typeof listChats>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
/**
 * @summary Get a list of chats
 */

export function useListChats<TData = Awaited<ReturnType<typeof listChats>>, TError = ErrorType<ProblemSchema>>(
  options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
 ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {

  const queryOptions = getListChatsQueryOptions(options)

  const query = useQuery(queryOptions, queryClient) as  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return { ...query, queryKey: queryOptions.queryKey };
}




/**
 * Creates a new conversation, streams the AI reply back via Server-Sent Events. The first event is `conversation_created` with the new conversation ID; subsequent `message` events carry incremental token chunks; a `done` event signals the final chunk; `saved` confirms the reply was persisted; `error` is emitted on failures.
 * @summary Create a new chat
 */
export type createChatResponse200 = {
  data: string
  status: 200
}

export type createChatResponse400 = {
  data: ProblemSchema
  status: 400
}

export type createChatResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createChatResponse500 = {
  data: ProblemSchema
  status: 500
}

export type createChatResponseSuccess = (createChatResponse200) & {
  headers: Headers;
};
export type createChatResponseError = (createChatResponse400 | createChatResponse401 | createChatResponse500) & {
  headers: Headers;
};

export type createChatResponse = (createChatResponseSuccess | createChatResponseError)

export const getCreateChatUrl = () => {




  return `/api/v1/chats`
}

export const createChat = async (chatRequest: ChatRequest, options?: RequestInit): Promise<createChatResponse> => {

  return customFetch<createChatResponse>(getCreateChatUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(
      chatRequest,)
  }
);}




export const getCreateChatMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof createChat>>, TError,{data: ChatRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): UseMutationOptions<Awaited<ReturnType<typeof createChat>>, TError,{data: ChatRequest}, TContext> => {

const mutationKey = ['createChat'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof createChat>>, {data: ChatRequest}> = (props) => {
          const {data} = props ?? {};

          return  createChat(data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type CreateChatMutationResult = NonNullable<Awaited<ReturnType<typeof createChat>>>
    export type CreateChatMutationBody = ChatRequest
    export type CreateChatMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Create a new chat
 */
export const useCreateChat = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof createChat>>, TError,{data: ChatRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient): UseMutationResult<
        Awaited<ReturnType<typeof createChat>>,
        TError,
        {data: ChatRequest},
        TContext
      > => {
      return useMutation(getCreateChatMutationOptions(options), queryClient);
    }
    /**
 * Returns a conversation and all its messages for the authenticated user.
 * @summary Get a specific chat
 */
export type getChatResponse200 = {
  data: ConversationDetail
  status: 200
}

export type getChatResponse401 = {
  data: ProblemSchema
  status: 401
}

export type getChatResponse404 = {
  data: ProblemSchema
  status: 404
}

export type getChatResponse500 = {
  data: ProblemSchema
  status: 500
}

export type getChatResponseSuccess = (getChatResponse200) & {
  headers: Headers;
};
export type getChatResponseError = (getChatResponse401 | getChatResponse404 | getChatResponse500) & {
  headers: Headers;
};

export type getChatResponse = (getChatResponseSuccess | getChatResponseError)

export const getGetChatUrl = (id: string,) => {




  return `/api/v1/chats/${id}`
}

export const getChat = async (id: string, options?: RequestInit): Promise<getChatResponse> => {

  return customFetch<getChatResponse>(getGetChatUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}





export const getGetChatQueryKey = (id: string,) => {
    return [
    `/api/v1/chats/${id}`
    ] as const;
    }


export const getGetChatQueryOptions = <TData = Awaited<ReturnType<typeof getChat>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getGetChatQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof getChat>>> = ({ signal }) => getChat(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: !!(id), ...queryOptions} as UseQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type GetChatQueryResult = NonNullable<Awaited<ReturnType<typeof getChat>>>
export type GetChatQueryError = ErrorType<ProblemSchema>


export function useGetChat<TData = Awaited<ReturnType<typeof getChat>>, TError = ErrorType<ProblemSchema>>(
 id: string, options: { query:Partial<UseQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData>> & Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof getChat>>,
          TError,
          Awaited<ReturnType<typeof getChat>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useGetChat<TData = Awaited<ReturnType<typeof getChat>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData>> & Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof getChat>>,
          TError,
          Awaited<ReturnType<typeof getChat>>
        > , 'initialData'
      >, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
export function useGetChat<TData = Awaited<ReturnType<typeof getChat>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
  ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> }
/**
 * @summary Get a specific chat
 */

export function useGetChat<TData = Awaited<ReturnType<typeof getChat>>, TError = ErrorType<ProblemSchema>>(
 id: string, options?: { query?:Partial<UseQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient
 ):  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {

  const queryOptions = getGetChatQueryOptions(id,options)

  const query = useQuery(queryOptions, queryClient) as  UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return { ...query, queryKey: queryOptions.queryKey };
}




/**
 * Appends a new user message to the conversation and streams the AI reply back via Server-Sent Events. Requires the conversation to belong to the authenticated user. SSE events emitted: `message` (token chunk), `done` (final token chunk), `saved` (conversation UUID confirming persistence), `error` (error message).
 * @summary Send a message in an existing chat
 */
export type sendChatMessageResponse200 = {
  data: string
  status: 200
}

export type sendChatMessageResponse400 = {
  data: ProblemSchema
  status: 400
}

export type sendChatMessageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type sendChatMessageResponse404 = {
  data: ProblemSchema
  status: 404
}

export type sendChatMessageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type sendChatMessageResponseSuccess = (sendChatMessageResponse200) & {
  headers: Headers;
};
export type sendChatMessageResponseError = (sendChatMessageResponse400 | sendChatMessageResponse401 | sendChatMessageResponse404 | sendChatMessageResponse500) & {
  headers: Headers;
};

export type sendChatMessageResponse = (sendChatMessageResponseSuccess | sendChatMessageResponseError)

export const getSendChatMessageUrl = (id: string,) => {




  return `/api/v1/chats/${id}`
}

export const sendChatMessage = async (id: string,
    messageRequest: MessageRequest, options?: RequestInit): Promise<sendChatMessageResponse> => {

  return customFetch<sendChatMessageResponse>(getSendChatMessageUrl(id),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(
      messageRequest,)
  }
);}




export const getSendChatMessageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError,{id: string;data: MessageRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): UseMutationOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError,{id: string;data: MessageRequest}, TContext> => {

const mutationKey = ['sendChatMessage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof sendChatMessage>>, {id: string;data: MessageRequest}> = (props) => {
          const {id,data} = props ?? {};

          return  sendChatMessage(id,data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type SendChatMessageMutationResult = NonNullable<Awaited<ReturnType<typeof sendChatMessage>>>
    export type SendChatMessageMutationBody = MessageRequest
    export type SendChatMessageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Send a message in an existing chat
 */
export const useSendChatMessage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:UseMutationOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError,{id: string;data: MessageRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: QueryClient): UseMutationResult<
        Awaited<ReturnType<typeof sendChatMessage>>,
        TError,
        {id: string;data: MessageRequest},
        TContext
      > => {
      return useMutation(getSendChatMessageMutationOptions(options), queryClient);
    }
