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
  ChatRequest,
  Conversation,
  ConversationDetail,
  MessageRequest,
  ProblemSchema
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



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

/**
 * Returns a list of the authenticated user's conversations ordered by most recently updated.
 * @summary Get a list of chats
 */
export const listChats = async ( options?: RequestInit): Promise<listChatsResponse> => {

  return customFetch<listChatsResponse>(getListChatsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}




export const getListChatsMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listChats>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof listChats>>, TError,void, TContext> => {

const mutationKey = ['listChats'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof listChats>>, void> = () => {


          return  listChats(requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type ListChatsMutationResult = NonNullable<Awaited<ReturnType<typeof listChats>>>

    export type ListChatsMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get a list of chats
 */
export const createListChats = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof listChats>>, TError,void, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof listChats>>,
        TError,
        void,
        TContext
      > => {
      return createMutation(() => ({ ...getListChatsMutationOptions(options?.()) }), queryClient);
    }
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

/**
 * Creates a new conversation, streams the AI reply back via Server-Sent Events. The first event is `conversation_created` with the new conversation ID; subsequent `message` events carry incremental token chunks; a `done` event signals the final chunk; `saved` confirms the reply was persisted; `error` is emitted on failures.
 * @summary Create a new chat
 */
export const createChat = async (chatRequest: ChatRequest, options?: RequestInit): Promise<createChatResponse> => {

  return customFetch<createChatResponse>(getCreateChatUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(chatRequest)
  }
);}





export const getCreateChatQueryKey = (chatRequest?: ChatRequest,) => {
    return [
    'POST', `/api/v1/chats`, chatRequest
    ] as const;
    }


export const getCreateChatQueryOptions = <TData = Awaited<ReturnType<typeof createChat>>, TError = ErrorType<ProblemSchema>>(chatRequest: ChatRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getCreateChatQueryKey(chatRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof createChat>>> = ({ signal }) => createChat(chatRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof createChat>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type CreateChatQueryResult = NonNullable<Awaited<ReturnType<typeof createChat>>>
export type CreateChatQueryError = ErrorType<ProblemSchema>


/**
 * @summary Create a new chat
 */

export function createCreateChat<TData = Awaited<ReturnType<typeof createChat>>, TError = ErrorType<ProblemSchema>>(
 chatRequest: () =>  ChatRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof createChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getCreateChatQueryOptions(chatRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






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

/**
 * Returns a conversation and all its messages for the authenticated user.
 * @summary Get a specific chat
 */
export const getChat = async (id: string, options?: RequestInit): Promise<getChatResponse> => {

  return customFetch<getChatResponse>(getGetChatUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}




export const getGetChatMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getChat>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof getChat>>, TError,{id: string}, TContext> => {

const mutationKey = ['getChat'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof getChat>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  getChat(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type GetChatMutationResult = NonNullable<Awaited<ReturnType<typeof getChat>>>

    export type GetChatMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Get a specific chat
 */
export const createGetChat = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof getChat>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof getChat>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getGetChatMutationOptions(options?.()) }), queryClient);
    }
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

/**
 * Appends a new user message to the conversation and streams the AI reply back via Server-Sent Events. Requires the conversation to belong to the authenticated user. SSE events emitted: `message` (token chunk), `done` (final token chunk), `saved` (conversation UUID confirming persistence), `error` (error message).
 * @summary Send a message in an existing chat
 */
export const sendChatMessage = async (id: string,
    messageRequest: MessageRequest, options?: RequestInit): Promise<sendChatMessageResponse> => {

  return customFetch<sendChatMessageResponse>(getSendChatMessageUrl(id),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(messageRequest)
  }
);}





export const getSendChatMessageQueryKey = (id: string,
    messageRequest?: MessageRequest,) => {
    return [
    'POST', `/api/v1/chats/${id}`, messageRequest
    ] as const;
    }


export const getSendChatMessageQueryOptions = <TData = Awaited<ReturnType<typeof sendChatMessage>>, TError = ErrorType<ProblemSchema>>(id: string,
    messageRequest: MessageRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getSendChatMessageQueryKey(id,messageRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof sendChatMessage>>> = ({ signal }) => sendChatMessage(id,messageRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type SendChatMessageQueryResult = NonNullable<Awaited<ReturnType<typeof sendChatMessage>>>
export type SendChatMessageQueryError = ErrorType<ProblemSchema>


/**
 * @summary Send a message in an existing chat
 */

export function createSendChatMessage<TData = Awaited<ReturnType<typeof sendChatMessage>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string,
    messageRequest: () =>  MessageRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getSendChatMessageQueryOptions(id(),
    messageRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






