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
  EditLatestMessageRequest,
  MessageRequest,
  ProblemSchema,
  UpdateChatRequest
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
 * Creates a new conversation and streams the AI reply via Server-Sent Events. Events: `conversation_created` (new conversation UUID, sent first), `reasoning` (chain-of-thought chunk, reasoning models only), `message` (answer token chunk), `saved` (conversation UUID confirming persistence, sent last), `error` (error message).
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
 * Appends a new user message to the conversation and streams the AI reply via Server-Sent Events. Requires the conversation to belong to the authenticated user. Events: `reasoning` (chain-of-thought chunk, reasoning models only), `message` (answer token chunk), `saved` (conversation UUID confirming persistence, sent last), `error` (error message).
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






export type deleteChatResponse204 = {
  data: void
  status: 204
}

export type deleteChatResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteChatResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteChatResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteChatResponseSuccess = (deleteChatResponse204) & {
  headers: Headers;
};
export type deleteChatResponseError = (deleteChatResponse401 | deleteChatResponse404 | deleteChatResponse500) & {
  headers: Headers;
};

export type deleteChatResponse = (deleteChatResponseSuccess | deleteChatResponseError)

export const getDeleteChatUrl = (id: string,) => {




  return `/api/v1/chats/${id}`
}

/**
 * Permanently deletes a conversation and all of its messages. Requires the conversation to belong to the authenticated user.
 * @summary Delete a chat
 */
export const deleteChat = async (id: string, options?: RequestInit): Promise<deleteChatResponse> => {

  return customFetch<deleteChatResponse>(getDeleteChatUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}





export const getDeleteChatQueryKey = (id: string,) => {
    return [
    'DELETE', `/api/v1/chats/${id}`
    ] as const;
    }


export const getDeleteChatQueryOptions = <TData = Awaited<ReturnType<typeof deleteChat>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getDeleteChatQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof deleteChat>>> = ({ signal }) => deleteChat(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof deleteChat>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type DeleteChatQueryResult = NonNullable<Awaited<ReturnType<typeof deleteChat>>>
export type DeleteChatQueryError = ErrorType<ProblemSchema>


/**
 * @summary Delete a chat
 */

export function createDeleteChat<TData = Awaited<ReturnType<typeof deleteChat>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getDeleteChatQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type updateChatResponse200 = {
  data: Conversation
  status: 200
}

export type updateChatResponse401 = {
  data: ProblemSchema
  status: 401
}

export type updateChatResponse404 = {
  data: ProblemSchema
  status: 404
}

export type updateChatResponse500 = {
  data: ProblemSchema
  status: 500
}

export type updateChatResponseSuccess = (updateChatResponse200) & {
  headers: Headers;
};
export type updateChatResponseError = (updateChatResponse401 | updateChatResponse404 | updateChatResponse500) & {
  headers: Headers;
};

export type updateChatResponse = (updateChatResponseSuccess | updateChatResponseError)

export const getUpdateChatUrl = (id: string,) => {




  return `/api/v1/chats/${id}`
}

/**
 * Updates mutable fields on a conversation owned by the authenticated user. Currently supports archiving and unarchiving.
 * @summary Update a chat
 */
export const updateChat = async (id: string,
    updateChatRequest: UpdateChatRequest, options?: RequestInit): Promise<updateChatResponse> => {

  return customFetch<updateChatResponse>(getUpdateChatUrl(id),
  {
    ...options,
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(updateChatRequest)
  }
);}





export const getUpdateChatQueryKey = (id: string,
    updateChatRequest?: UpdateChatRequest,) => {
    return [
    'PATCH', `/api/v1/chats/${id}`, updateChatRequest
    ] as const;
    }


export const getUpdateChatQueryOptions = <TData = Awaited<ReturnType<typeof updateChat>>, TError = ErrorType<ProblemSchema>>(id: string,
    updateChatRequest: UpdateChatRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getUpdateChatQueryKey(id,updateChatRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof updateChat>>> = ({ signal }) => updateChat(id,updateChatRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof updateChat>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type UpdateChatQueryResult = NonNullable<Awaited<ReturnType<typeof updateChat>>>
export type UpdateChatQueryError = ErrorType<ProblemSchema>


/**
 * @summary Update a chat
 */

export function createUpdateChat<TData = Awaited<ReturnType<typeof updateChat>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string,
    updateChatRequest: () =>  UpdateChatRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof updateChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getUpdateChatQueryOptions(id(),
    updateChatRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type editLatestChatMessageResponse200 = {
  data: string
  status: 200
}

export type editLatestChatMessageResponse400 = {
  data: ProblemSchema
  status: 400
}

export type editLatestChatMessageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type editLatestChatMessageResponse404 = {
  data: ProblemSchema
  status: 404
}

export type editLatestChatMessageResponse409 = {
  data: ProblemSchema
  status: 409
}

export type editLatestChatMessageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type editLatestChatMessageResponseSuccess = (editLatestChatMessageResponse200) & {
  headers: Headers;
};
export type editLatestChatMessageResponseError = (editLatestChatMessageResponse400 | editLatestChatMessageResponse401 | editLatestChatMessageResponse404 | editLatestChatMessageResponse409 | editLatestChatMessageResponse500) & {
  headers: Headers;
};

export type editLatestChatMessageResponse = (editLatestChatMessageResponseSuccess | editLatestChatMessageResponseError)

export const getEditLatestChatMessageUrl = (id: string,) => {




  return `/api/v1/chats/${id}/messages/latest`
}

/**
 * Replaces the content of the conversation's most recent user message, discards the assistant reply (and any later turns) that followed it, then regenerates and streams a fresh reply via Server-Sent Events. Only the latest message can be edited, since editing an earlier one would require regenerating everything after it. Events: `reasoning` (chain-of-thought chunk, reasoning models only), `message` (answer token chunk), `saved` (conversation UUID confirming persistence, sent last), `error` (error message).
 * @summary Edit the latest message in a chat
 */
export const editLatestChatMessage = async (id: string,
    editLatestMessageRequest: EditLatestMessageRequest, options?: RequestInit): Promise<editLatestChatMessageResponse> => {

  return customFetch<editLatestChatMessageResponse>(getEditLatestChatMessageUrl(id),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(editLatestMessageRequest)
  }
);}





export const getEditLatestChatMessageQueryKey = (id: string,
    editLatestMessageRequest?: EditLatestMessageRequest,) => {
    return [
    'POST', `/api/v1/chats/${id}/messages/latest`, editLatestMessageRequest
    ] as const;
    }


export const getEditLatestChatMessageQueryOptions = <TData = Awaited<ReturnType<typeof editLatestChatMessage>>, TError = ErrorType<ProblemSchema>>(id: string,
    editLatestMessageRequest: EditLatestMessageRequest, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof editLatestChatMessage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getEditLatestChatMessageQueryKey(id,editLatestMessageRequest);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof editLatestChatMessage>>> = ({ signal }) => editLatestChatMessage(id,editLatestMessageRequest, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof editLatestChatMessage>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type EditLatestChatMessageQueryResult = NonNullable<Awaited<ReturnType<typeof editLatestChatMessage>>>
export type EditLatestChatMessageQueryError = ErrorType<ProblemSchema>


/**
 * @summary Edit the latest message in a chat
 */

export function createEditLatestChatMessage<TData = Awaited<ReturnType<typeof editLatestChatMessage>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string,
    editLatestMessageRequest: () =>  EditLatestMessageRequest, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof editLatestChatMessage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getEditLatestChatMessageQueryOptions(id(),
    editLatestMessageRequest(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type deleteChatMessageResponse204 = {
  data: void
  status: 204
}

export type deleteChatMessageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteChatMessageResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteChatMessageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteChatMessageResponseSuccess = (deleteChatMessageResponse204) & {
  headers: Headers;
};
export type deleteChatMessageResponseError = (deleteChatMessageResponse401 | deleteChatMessageResponse404 | deleteChatMessageResponse500) & {
  headers: Headers;
};

export type deleteChatMessageResponse = (deleteChatMessageResponseSuccess | deleteChatMessageResponseError)

export const getDeleteChatMessageUrl = (id: string,
    messageId: string,) => {




  return `/api/v1/chats/${id}/messages/${messageId}`
}

/**
 * Permanently deletes the given message and all messages that follow it in the conversation, keeping every earlier message intact. Requires the conversation to belong to the authenticated user.
 * @summary Delete a message and everything after it
 */
export const deleteChatMessage = async (id: string,
    messageId: string, options?: RequestInit): Promise<deleteChatMessageResponse> => {

  return customFetch<deleteChatMessageResponse>(getDeleteChatMessageUrl(id,messageId),
  {
    ...options,
    method: 'DELETE'


  }
);}





export const getDeleteChatMessageQueryKey = (id: string,
    messageId: string,) => {
    return [
    'DELETE', `/api/v1/chats/${id}/messages/${messageId}`
    ] as const;
    }


export const getDeleteChatMessageQueryOptions = <TData = Awaited<ReturnType<typeof deleteChatMessage>>, TError = ErrorType<ProblemSchema>>(id: string,
    messageId: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteChatMessage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getDeleteChatMessageQueryKey(id,messageId);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof deleteChatMessage>>> = ({ signal }) => deleteChatMessage(id,messageId, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined && messageId !== null && messageId !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof deleteChatMessage>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type DeleteChatMessageQueryResult = NonNullable<Awaited<ReturnType<typeof deleteChatMessage>>>
export type DeleteChatMessageQueryError = ErrorType<ProblemSchema>


/**
 * @summary Delete a message and everything after it
 */

export function createDeleteChatMessage<TData = Awaited<ReturnType<typeof deleteChatMessage>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string,
    messageId: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof deleteChatMessage>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getDeleteChatMessageQueryOptions(id(),
    messageId(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






