// @ts-nocheck

import type {
    CreateMutationOptions,
    CreateMutationResult,
    CreateQueryOptions,
    CreateQueryResult,
    DataTag,
    DefinedCreateQueryResult,
    DefinedInitialDataOptions,
    MutationFunction,
    QueryClient,
    QueryFunction,
    QueryKey,
    UndefinedInitialDataOptions,
} from "@tanstack/svelte-query";
import { createQuery } from "@tanstack/svelte-query";
import type { ErrorType } from "$lib/http/orval-mutator.js";

import { customFetch } from "$lib/http/orval-mutator.js";
import type { ProblemSchema } from "../models/index.js";

type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];

/**
 * @summary Get a list of organization
 */
export type listOrgsResponse200 = {
    data: undefined;
    status: 200;
};

export type listOrgsResponse401 = {
    data: ProblemSchema;
    status: 401;
};

export type listOrgsResponse500 = {
    data: ProblemSchema;
    status: 500;
};

export type listOrgsResponseSuccess = listOrgsResponse200 & {
    headers: Headers;
};
export type listOrgsResponseError = (
    | listOrgsResponse401
    | listOrgsResponse500
) & {
    headers: Headers;
};

export type listOrgsResponse = listOrgsResponseSuccess | listOrgsResponseError;

export const getListOrgsUrl = () => {
    return `/api/v1/orgs`;
};

export const listOrgs = async (
    options?: RequestInit,
): Promise<listOrgsResponse> => {
    return customFetch<listOrgsResponse>(getListOrgsUrl(), {
        ...options,
        method: "GET",
    });
};

export const getListOrgsQueryKey = () => {
    return [`/api/v1/orgs`] as const;
};

export const getListOrgsQueryOptions = <
    TData = Awaited<ReturnType<typeof listOrgs>>,
    TError = ErrorType<ProblemSchema>,
>(options?: {
    query?: Partial<
        CreateQueryOptions<Awaited<ReturnType<typeof listOrgs>>, TError, TData>
    >;
    request?: SecondParameter<typeof customFetch>;
}) => {
    const { query: queryOptions, request: requestOptions } = options ?? {};

    const queryKey = queryOptions?.queryKey ?? getListOrgsQueryKey();

    const queryFn: QueryFunction<Awaited<ReturnType<typeof listOrgs>>> = ({
        signal,
    }) => listOrgs({ signal, ...requestOptions });

    return { queryKey, queryFn, ...queryOptions } as CreateQueryOptions<
        Awaited<ReturnType<typeof listOrgs>>,
        TError,
        TData
    > & { queryKey: DataTag<QueryKey, TData, TError> };
};

export type ListOrgsQueryResult = NonNullable<
    Awaited<ReturnType<typeof listOrgs>>
>;
export type ListOrgsQueryError = ErrorType<ProblemSchema>;

export function useListOrgs<
    TData = Awaited<ReturnType<typeof listOrgs>>,
    TError = ErrorType<ProblemSchema>,
>(
    options: {
        query: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof listOrgs>>,
                TError,
                TData
            >
        > &
            Pick<
                DefinedInitialDataOptions<
                    Awaited<ReturnType<typeof listOrgs>>,
                    TError,
                    Awaited<ReturnType<typeof listOrgs>>
                >,
                "initialData"
            >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): DefinedCreateQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData, TError>;
};
export function useListOrgs<
    TData = Awaited<ReturnType<typeof listOrgs>>,
    TError = ErrorType<ProblemSchema>,
>(
    options?: {
        query?: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof listOrgs>>,
                TError,
                TData
            >
        > &
            Pick<
                UndefinedInitialDataOptions<
                    Awaited<ReturnType<typeof listOrgs>>,
                    TError,
                    Awaited<ReturnType<typeof listOrgs>>
                >,
                "initialData"
            >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData, TError>;
};
export function useListOrgs<
    TData = Awaited<ReturnType<typeof listOrgs>>,
    TError = ErrorType<ProblemSchema>,
>(
    options?: {
        query?: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof listOrgs>>,
                TError,
                TData
            >
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData, TError>;
};
/**
 * @summary Get a list of organization
 */

export function useListOrgs<
    TData = Awaited<ReturnType<typeof listOrgs>>,
    TError = ErrorType<ProblemSchema>,
>(
    options?: {
        query?: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof listOrgs>>,
                TError,
                TData
            >
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData, TError>;
} {
    const queryOptions = getListOrgsQueryOptions(options);

    const query = createQuery(queryOptions, queryClient) as CreateQueryResult<
        TData,
        TError
    > & { queryKey: DataTag<QueryKey, TData, TError> };

    return { ...query, queryKey: queryOptions.queryKey };
}

/**
 * @summary Create a new organization
 */
export type createOrgResponse200 = {
    data: undefined;
    status: 200;
};

export type createOrgResponse401 = {
    data: ProblemSchema;
    status: 401;
};

export type createOrgResponse500 = {
    data: ProblemSchema;
    status: 500;
};

export type createOrgResponseSuccess = createOrgResponse200 & {
    headers: Headers;
};
export type createOrgResponseError = (
    | createOrgResponse401
    | createOrgResponse500
) & {
    headers: Headers;
};

export type createOrgResponse =
    | createOrgResponseSuccess
    | createOrgResponseError;

export const getCreateOrgUrl = () => {
    return `/api/v1/orgs`;
};

export const createOrg = async (
    options?: RequestInit,
): Promise<createOrgResponse> => {
    return customFetch<createOrgResponse>(getCreateOrgUrl(), {
        ...options,
        method: "POST",
    });
};

export const getCreateOrgMutationOptions = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(options?: {
    mutation?: CreateMutationOptions<
        Awaited<ReturnType<typeof createOrg>>,
        TError,
        void,
        TContext
    >;
    request?: SecondParameter<typeof customFetch>;
}): CreateMutationOptions<
    Awaited<ReturnType<typeof createOrg>>,
    TError,
    void,
    TContext
> => {
    const mutationKey = ["createOrg"];
    const { mutation: mutationOptions, request: requestOptions } = options
        ? options.mutation &&
          "mutationKey" in options.mutation &&
          options.mutation.mutationKey
            ? options
            : { ...options, mutation: { ...options.mutation, mutationKey } }
        : { mutation: { mutationKey }, request: undefined };

    const mutationFn: MutationFunction<
        Awaited<ReturnType<typeof createOrg>>,
        void
    > = () => {
        return createOrg(requestOptions);
    };

    return { mutationFn, ...mutationOptions };
};

export type CreateOrgMutationResult = NonNullable<
    Awaited<ReturnType<typeof createOrg>>
>;

export type CreateOrgMutationError = ErrorType<ProblemSchema>;

/**
 * @summary Create a new organization
 */
export const useCreateOrg = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(
    options?: {
        mutation?: CreateMutationOptions<
            Awaited<ReturnType<typeof createOrg>>,
            TError,
            void,
            TContext
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateMutationResult<
    Awaited<ReturnType<typeof createOrg>>,
    TError,
    void,
    TContext
> => {
    return useMutation(getCreateOrgMutationOptions(options), queryClient);
};
/**
 * @summary Get a specific organization
 */
export type getOrgResponse200 = {
    data: undefined;
    status: 200;
};

export type getOrgResponse401 = {
    data: ProblemSchema;
    status: 401;
};

export type getOrgResponse404 = {
    data: ProblemSchema;
    status: 404;
};

export type getOrgResponse500 = {
    data: ProblemSchema;
    status: 500;
};

export type getOrgResponseSuccess = getOrgResponse200 & {
    headers: Headers;
};
export type getOrgResponseError = (
    | getOrgResponse401
    | getOrgResponse404
    | getOrgResponse500
) & {
    headers: Headers;
};

export type getOrgResponse = getOrgResponseSuccess | getOrgResponseError;

export const getGetOrgUrl = (id: string) => {
    return `/api/v1/orgs/${id}`;
};

export const getOrg = async (
    id: string,
    options?: RequestInit,
): Promise<getOrgResponse> => {
    return customFetch<getOrgResponse>(getGetOrgUrl(id), {
        ...options,
        method: "GET",
    });
};

export const getGetOrgQueryKey = (id: string) => {
    return [`/api/v1/orgs/${id}`] as const;
};

export const getGetOrgQueryOptions = <
    TData = Awaited<ReturnType<typeof getOrg>>,
    TError = ErrorType<ProblemSchema>,
>(
    id: string,
    options?: {
        query?: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof getOrg>>,
                TError,
                TData
            >
        >;
        request?: SecondParameter<typeof customFetch>;
    },
) => {
    const { query: queryOptions, request: requestOptions } = options ?? {};

    const queryKey = queryOptions?.queryKey ?? getGetOrgQueryKey(id);

    const queryFn: QueryFunction<Awaited<ReturnType<typeof getOrg>>> = ({
        signal,
    }) => getOrg(id, { signal, ...requestOptions });

    return {
        queryKey,
        queryFn,
        enabled: !!id,
        ...queryOptions,
    } as CreateQueryOptions<
        Awaited<ReturnType<typeof getOrg>>,
        TError,
        TData
    > & { queryKey: DataTag<QueryKey, TData, TError> };
};

export type GetOrgQueryResult = NonNullable<Awaited<ReturnType<typeof getOrg>>>;
export type GetOrgQueryError = ErrorType<ProblemSchema>;

export function useGetOrg<
    TData = Awaited<ReturnType<typeof getOrg>>,
    TError = ErrorType<ProblemSchema>,
>(
    id: string,
    options: {
        query: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof getOrg>>,
                TError,
                TData
            >
        > &
            Pick<
                DefinedInitialDataOptions<
                    Awaited<ReturnType<typeof getOrg>>,
                    TError,
                    Awaited<ReturnType<typeof getOrg>>
                >,
                "initialData"
            >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): DefinedCreateQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData, TError>;
};
export function useGetOrg<
    TData = Awaited<ReturnType<typeof getOrg>>,
    TError = ErrorType<ProblemSchema>,
>(
    id: string,
    options?: {
        query?: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof getOrg>>,
                TError,
                TData
            >
        > &
            Pick<
                UndefinedInitialDataOptions<
                    Awaited<ReturnType<typeof getOrg>>,
                    TError,
                    Awaited<ReturnType<typeof getOrg>>
                >,
                "initialData"
            >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData, TError>;
};
export function useGetOrg<
    TData = Awaited<ReturnType<typeof getOrg>>,
    TError = ErrorType<ProblemSchema>,
>(
    id: string,
    options?: {
        query?: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof getOrg>>,
                TError,
                TData
            >
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData, TError>;
};
/**
 * @summary Get a specific organization
 */

export function useGetOrg<
    TData = Awaited<ReturnType<typeof getOrg>>,
    TError = ErrorType<ProblemSchema>,
>(
    id: string,
    options?: {
        query?: Partial<
            CreateQueryOptions<
                Awaited<ReturnType<typeof getOrg>>,
                TError,
                TData
            >
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData, TError>;
} {
    const queryOptions = getGetOrgQueryOptions(id, options);

    const query = createQuery(queryOptions, queryClient) as CreateQueryResult<
        TData,
        TError
    > & { queryKey: DataTag<QueryKey, TData, TError> };

    return { ...query, queryKey: queryOptions.queryKey };
}

/**
 * @summary Delete a new organization
 */
export type deleteOrgResponse200 = {
    data: undefined;
    status: 200;
};

export type deleteOrgResponse401 = {
    data: ProblemSchema;
    status: 401;
};

export type deleteOrgResponse404 = {
    data: ProblemSchema;
    status: 404;
};

export type deleteOrgResponse500 = {
    data: ProblemSchema;
    status: 500;
};

export type deleteOrgResponseSuccess = deleteOrgResponse200 & {
    headers: Headers;
};
export type deleteOrgResponseError = (
    | deleteOrgResponse401
    | deleteOrgResponse404
    | deleteOrgResponse500
) & {
    headers: Headers;
};

export type deleteOrgResponse =
    | deleteOrgResponseSuccess
    | deleteOrgResponseError;

export const getDeleteOrgUrl = (id: string) => {
    return `/api/v1/orgs/${id}`;
};

export const deleteOrg = async (
    id: string,
    options?: RequestInit,
): Promise<deleteOrgResponse> => {
    return customFetch<deleteOrgResponse>(getDeleteOrgUrl(id), {
        ...options,
        method: "DELETE",
    });
};

export const getDeleteOrgMutationOptions = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(options?: {
    mutation?: CreateMutationOptions<
        Awaited<ReturnType<typeof deleteOrg>>,
        TError,
        { id: string },
        TContext
    >;
    request?: SecondParameter<typeof customFetch>;
}): CreateMutationOptions<
    Awaited<ReturnType<typeof deleteOrg>>,
    TError,
    { id: string },
    TContext
> => {
    const mutationKey = ["deleteOrg"];
    const { mutation: mutationOptions, request: requestOptions } = options
        ? options.mutation &&
          "mutationKey" in options.mutation &&
          options.mutation.mutationKey
            ? options
            : { ...options, mutation: { ...options.mutation, mutationKey } }
        : { mutation: { mutationKey }, request: undefined };

    const mutationFn: MutationFunction<
        Awaited<ReturnType<typeof deleteOrg>>,
        { id: string }
    > = (props) => {
        const { id } = props ?? {};

        return deleteOrg(id, requestOptions);
    };

    return { mutationFn, ...mutationOptions };
};

export type DeleteOrgMutationResult = NonNullable<
    Awaited<ReturnType<typeof deleteOrg>>
>;

export type DeleteOrgMutationError = ErrorType<ProblemSchema>;

/**
 * @summary Delete a new organization
 */
export const useDeleteOrg = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(
    options?: {
        mutation?: CreateMutationOptions<
            Awaited<ReturnType<typeof deleteOrg>>,
            TError,
            { id: string },
            TContext
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateMutationResult<
    Awaited<ReturnType<typeof deleteOrg>>,
    TError,
    { id: string },
    TContext
> => {
    return useMutation(getDeleteOrgMutationOptions(options), queryClient);
};
/**
 * @summary Update a specific organization
 */
export type updateOrgResponse200 = {
    data: undefined;
    status: 200;
};

export type updateOrgResponse401 = {
    data: ProblemSchema;
    status: 401;
};

export type updateOrgResponse404 = {
    data: ProblemSchema;
    status: 404;
};

export type updateOrgResponse500 = {
    data: ProblemSchema;
    status: 500;
};

export type updateOrgResponseSuccess = updateOrgResponse200 & {
    headers: Headers;
};
export type updateOrgResponseError = (
    | updateOrgResponse401
    | updateOrgResponse404
    | updateOrgResponse500
) & {
    headers: Headers;
};

export type updateOrgResponse =
    | updateOrgResponseSuccess
    | updateOrgResponseError;

export const getUpdateOrgUrl = (id: string) => {
    return `/api/v1/orgs/${id}`;
};

export const updateOrg = async (
    id: string,
    options?: RequestInit,
): Promise<updateOrgResponse> => {
    return customFetch<updateOrgResponse>(getUpdateOrgUrl(id), {
        ...options,
        method: "PATCH",
    });
};

export const getUpdateOrgMutationOptions = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(options?: {
    mutation?: CreateMutationOptions<
        Awaited<ReturnType<typeof updateOrg>>,
        TError,
        { id: string },
        TContext
    >;
    request?: SecondParameter<typeof customFetch>;
}): CreateMutationOptions<
    Awaited<ReturnType<typeof updateOrg>>,
    TError,
    { id: string },
    TContext
> => {
    const mutationKey = ["updateOrg"];
    const { mutation: mutationOptions, request: requestOptions } = options
        ? options.mutation &&
          "mutationKey" in options.mutation &&
          options.mutation.mutationKey
            ? options
            : { ...options, mutation: { ...options.mutation, mutationKey } }
        : { mutation: { mutationKey }, request: undefined };

    const mutationFn: MutationFunction<
        Awaited<ReturnType<typeof updateOrg>>,
        { id: string }
    > = (props) => {
        const { id } = props ?? {};

        return updateOrg(id, requestOptions);
    };

    return { mutationFn, ...mutationOptions };
};

export type UpdateOrgMutationResult = NonNullable<
    Awaited<ReturnType<typeof updateOrg>>
>;

export type UpdateOrgMutationError = ErrorType<ProblemSchema>;

/**
 * @summary Update a specific organization
 */
export const useUpdateOrg = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(
    options?: {
        mutation?: CreateMutationOptions<
            Awaited<ReturnType<typeof updateOrg>>,
            TError,
            { id: string },
            TContext
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateMutationResult<
    Awaited<ReturnType<typeof updateOrg>>,
    TError,
    { id: string },
    TContext
> => {
    return useMutation(getUpdateOrgMutationOptions(options), queryClient);
};
