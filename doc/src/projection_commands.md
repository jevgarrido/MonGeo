# Projection Mode Commands

In projection mode one is only allowed to draw **points**, **lines** and **circles**.

These points and lines are drawn in the plane of the observer/user (you), and they represent projections or traces of geometric objects on specified projection planes.

As they are drawn, the points and lines are not associated with any concrete object, they simply exist on their own. The association of projections and traces to form concrete 3D objects must be done explicitly.

Therefore, in projection mode, the projections and traces come first and they define the geometric objects implicitly.

## Aliases for the Projection Planes
In projection mode, one must specify the projection plane in which one wishes to draw.
For that purpose, when drawing/creating a new point, or line, one should press the associated character or number alias of the intended plane.

Here is the list of aliases of the projection planes:

    v   0   View Plane
    f   1   Frontal Plane
    h   2   Horizontal Plane
    p   3   Profile Plane

In what follows, the place-holder for the projection plane alias is defined as ```[pp]```, which in practice will be either the character or the number associated with the intended plane.

One may also omit the alias when issuing a command. In which case the new object will be drawn in the View Plane by default.


## Drawing Points
We shall use the character ```.``` (point) as the trigger command to draw/create a new point.

    [pp] . [line] [line]                    intersection of two lines.
           
    [pp] . [circ] [line or circ] <Ctrl> Enter     
                                            intersection of a circumference and another
                                            curve (line or circumference).
                                            Optional Ctrl key to resolve ambiguity.

## Drawing Lines
We shall use the character ```-``` (dash) as the trigger command to draw/create a new line.

    [pp] - [point] [point]                  passing through two points.
         
    [pp] - [point] [line]                   passing through a point at 90 degrees with a
                                            line.
           
    [pp] - [point] (angle) [line] <Ctrl> Enter
                                            passing through a point at a specified angle
                                            with a line.
                                            Optional Ctrl key to resolve ambiguity.
           
    [pp] - [line] (length) <Ctrl> Enter     parallel at a distance (length) from a line.
                                            Optional Ctrl key to resolve ambiguity.


## Drawing Circumferences
We shall use the character ```<``` (open angle bracket) as the trigger command to draw/create a new circumference.
    
    [pp] < [point 1] [point 2]              center in point 1 and containing point 2.

    [pp] < [point] (length) Enter           center in a point with a given radus.



## Labels
Every element that is drawn in projection mode is automatically given a label.
This label is composed of 3 alphabetic characters. The generic format is given below using regular expression notation:

    [a-zA-Z]{2}

**Examples:** ```Gf``` ```BK``` ```xg```


## Associating Projections to Actual Objects
Define a concrete point, line, circumference or plane using the *name* command, ```n```:

    n [point or line or circ] [point or line or circ] = [a-z][a-z][a-z]

The newly named object can be a point, a line a circumference or a plane.

Only named objects can be used/referenced in Algebraic mode.


## Generic guidelines
* Lengths should be provided in cm (centimeters)
* Angles should be provided in degrees.

