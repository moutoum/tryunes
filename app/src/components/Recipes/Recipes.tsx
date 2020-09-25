import React, {HTMLAttributes, useCallback} from 'react';
import RecipeCard from '../RecipeCard';
import classes from './Recipes.module.css';

interface RecipesProps {
    recipes: Array<Recipe>
    onRecipeClicked?: (recipeId: number) => void
}

type Props = RecipesProps & HTMLAttributes<HTMLElement>;

const Recipes: React.FC<Props> = (props) => {
    const {recipes, onRecipeClicked, className, ...others} = props;
    const createButtonClickCb = useCallback(
        (recipeId: number) => () => onRecipeClicked!(recipeId),
        [onRecipeClicked]);

    return (
        <div className={[classes.container, className].join(" ")} {...others}>
            {recipes.map((recipe) => <RecipeCard
                key={recipe.id}
                recipe={recipe}
                onButtonClicked={createButtonClickCb(recipe.id)}
                style={{ marginBottom: 'calc(var(--unit) * 2)'}}
            />)}
        </div>
    );
};

export default Recipes;