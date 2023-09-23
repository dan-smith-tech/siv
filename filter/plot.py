import matplotlib.pyplot as plt


def draw_graph(pixels, labels, centroids, title):
    fig = plt.figure()
    ax = fig.add_subplot(projection="3d")

    ax.set_xlabel("Red")
    ax.set_ylabel("Green")
    ax.set_zlabel("Blue")

    colors = ((0, 0, 1), (0, 1, 0), (1, 0, 0))
    colors_full = [colors[int(i)] for i in labels]

    ax.scatter(
        pixels[:, 0],
        pixels[:, 1],
        pixels[:, 2],
        marker="o",
        c=colors_full,
        s=10,
        zorder=-1,
    )
    ax.scatter(
        centroids[:, 0],
        centroids[:, 1],
        centroids[:, 2],
        marker="*",
        color=colors,
        s=500,
        zorder=1,
    )

    ax.set_title(title)

    plt.show()
