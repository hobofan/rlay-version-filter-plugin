const rlay = require('@rlay/web3-rlay');

const label = value => ({
  type: 'Annotation',
  property: '*labelAnnotationProperty',
  value,
});

module.exports = {
  version: '2',
  imports: {
    ...rlay.builtins,
  },
  entities: {
    urlLabel: label('Univeral Resource Location'),
    urlDataProperty: {
      type: 'DataProperty',
      annotations: ['*urlLabel'],
    },

    versioningLabel: label('Custom data property versioning'),
    versioningAnnotationProperty: {
      type: 'AnnotationProperty',
      annotations: ['*versioningLabel'],
    }
  },
};
