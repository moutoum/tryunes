import React, {FC, useCallback, useState} from "react";
import {Button, Form} from "react-bootstrap";

interface Props {
    onCreate: (ingredient: { name: string, image: string }) => void,
    onCancel: () => void
}

const IngredientCreationForm: FC<Props> = ({onCancel, onCreate}) => {
    const [name, setName] = useState("");
    const [image, setImage] = useState("");

    const handleSubmit = useCallback((e) => {
        e.preventDefault();
        onCreate({name, image})
    }, [name, image, onCreate]);
    const handleNameChange = useCallback((e) => {
        e.preventDefault();
        setName(e.target.value);
    }, [setName]);
    const handleImageChange = useCallback((e) => {
        e.preventDefault();
        setImage(e.target.value);
    }, [setImage]);

    return (
        <Form onSubmit={handleSubmit}>
            <Form.Group>
                <Form.Label>Nom</Form.Label>
                <Form.Control
                    size="sm"
                    type="text"
                    placeholder="Nom de l'ingredient.."
                    onChange={handleNameChange}
                    value={name}
                />
            </Form.Group>
            <Form.Group>
                <Form.Label>Image</Form.Label>
                <Form.Control
                    size="sm"
                    type="text"
                    placeholder="Insérer le lien de l'image..."
                    onChange={handleImageChange}
                    value={image}
                />
            </Form.Group>
            <div className="d-flex justify-content-end">
                <Button size="sm" variant="light" onClick={onCancel} className="mr-2">Annuler</Button>
                <Button size="sm" variant="success" type="submit">Créer</Button>
            </div>
        </Form>
    );
}

export default IngredientCreationForm;