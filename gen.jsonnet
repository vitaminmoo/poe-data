local j(x) = std.manifestJsonEx(x, ' ');
{
  'data/config.json': j(import 'config.libsonnet'),
}
+ import 'tables.libsonnet'
