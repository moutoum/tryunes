export type Action =
    | { type: 'FETCH_RECIPES_REQUEST' }
    | { type: 'FETCH_RECIPES_SUCCESS', recipes: Array<Recipe> }
    | { type: 'FETCH_RECIPES_FAILURE', error: string };
