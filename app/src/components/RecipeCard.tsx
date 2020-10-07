import React, {FC} from 'react';
import {Badge, Card, Col, Row} from "react-bootstrap";

interface Props {
    recipe: Recipe
}

const RecipeCard: FC<Props> = ({recipe}) => {
    return (
        <Card>
            <Card.Body>
                <Card.Title>{recipe.name} <Badge
                    variant={recipe.price > 10 ? "warning" : "info"}>{recipe.price} €</Badge></Card.Title>
                <Card.Text>{recipe.description}</Card.Text>
            </Card.Body>
            <Card.Img variant="top" src={recipe.image}/>
            <Card.Footer>
                <Row>
                    <Col>
                        <small>
                            <div><b>Temps de preparation</b></div>
                            <div>{recipe.preparation_duration} min</div>
                        </small>
                    </Col>
                    <Col>
                        <small>
                            <div><b>Temps de cuisson</b></div>
                            <div>{recipe.cooking_duration} min</div>
                        </small>
                    </Col>
                </Row>
            </Card.Footer>
        </Card>
    );
};

export default RecipeCard;