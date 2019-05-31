
import React from 'react';
import Grid from '@material-ui/core/Grid';
import Typography from '@material-ui/core/Typography';

const NotFound: React.FC = () => {
    return (
        <Grid
            container
            direction="column"
            justify="center"
            alignItems="center"
            className="centered-grid">

            <Typography variant="h1"> 404 not found </Typography>
        </Grid>
    );
}

export default NotFound;