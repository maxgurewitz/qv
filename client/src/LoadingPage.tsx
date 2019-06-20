import React from 'react';
import Grid from '@material-ui/core/Grid';
import CircularProgress from '@material-ui/core/CircularProgress';

const LoadingPage: React.FC = () => {
  return (
    <Grid
      container
      direction="column"
      justify="center"
      alignItems="center"
      className="centered-grid">
      <CircularProgress />
    </Grid>
  );
}

export default LoadingPage;