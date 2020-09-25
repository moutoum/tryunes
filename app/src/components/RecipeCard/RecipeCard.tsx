import React, {HTMLAttributes} from 'react';
import classes from './RecipeCard.module.css';
import Button from '../Button';

interface RecipeProps {
    recipe: Recipe
    onButtonClicked?: () => void
}

type Props = RecipeProps & HTMLAttributes<HTMLElement>;

const RecipeCard: React.FC<Props> = ({recipe, onButtonClicked, className, ...others}) => (
    <div className={[classes.container, className].join(" ")} {...others}>
        <div className={classes.card}>
            <img src={recipe.image} alt={recipe.name}/>
            <div className={classes.content}>
                <h2>{recipe.name}</h2>
                <p>{recipe.description}</p>
                <div className={classes.additionalInformationContainer}>
                    <div className={classes.additionalInformation}>
                        <div>Temps de préparation</div>
                        <div>{recipe.preparation_duration / 60} min</div>
                    </div>
                    <div className={classes.additionalInformation}>
                        <div>Temps de cuisson</div>
                        <div>{recipe.cooking_duration / 60} min</div>
                    </div>
                    <div className={classes.additionalInformation}>
                        <div>Prix</div>
                        <div>{recipe.price} €</div>
                    </div>
                </div>
            </div>
        </div>
        <Button className={classes.button} label="Voir la recette" icon="chevron_right" onClick={onButtonClicked}/>
    </div>
)

export default RecipeCard;