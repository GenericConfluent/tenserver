# What is this?
It has a similar purpose to GenericConfluent/daimon which is basically to
allow ordinary people to run models that far exceed their computers hardware
limits by pooling their computing resources together with other people and
distributing the model across multiple devices. Neither project is finished,
but by the end you should be able to fairly conveniently load up a model across
multiple devices and have it run inference. 

The main differences between tenserver and daimon are their approaches towards
distribution. tenserver basically serves to be a semi-centralized approach to 
communication and distribution. Whereas daimon is extremly unorganized, this is
by design. If you are looking to run inference on large models tenserver is
probably what you're going to want.

# Security
No.
