#include <alsa/asoundlib.h>

const char *CARD = "default";
const char *SELEM_NAME = "Master";

typedef enum
{
    OK = 0,
    NO_DEFAULT = 1,
    NO_MASTER = 2,
    NO_CHANNEL = 3,
    NO_SWITCH = 4,
} exit_code;

int get_volume(long *outvol)
{
    long min, max;

    snd_mixer_t *handle;
    snd_mixer_selem_id_t *sid;

    snd_mixer_open(&handle, 0);

    if (snd_mixer_attach(handle, CARD) != 0)
    {
        snd_mixer_close(handle);
        return NO_DEFAULT;
    }

    snd_mixer_selem_register(handle, NULL, NULL);
    snd_mixer_load(handle);

    snd_mixer_selem_id_alloca(&sid);
    snd_mixer_selem_id_set_index(sid, 0);
    snd_mixer_selem_id_set_name(sid, SELEM_NAME);
    snd_mixer_elem_t *elem = snd_mixer_find_selem(handle, sid);

    if (!elem)
    {
        snd_mixer_close(handle);
        return NO_MASTER;
    }

    snd_mixer_selem_get_playback_volume_range(elem, &min, &max);

    if (snd_mixer_selem_get_playback_volume(elem, 0, outvol) != 0)
    {
        snd_mixer_close(handle);
        return NO_CHANNEL;
    }

    *outvol = 100 * (*outvol) / max;
    if (*outvol > 100)
    {
        *outvol = 100;
    }
    snd_mixer_close(handle);
    return OK;
}

int set_volume(long volume)
{
    long min, max;
    snd_mixer_t *handle;
    snd_mixer_selem_id_t *sid;

    snd_mixer_open(&handle, 0);

    if (snd_mixer_attach(handle, CARD) != 0)
    {
        snd_mixer_close(handle);
        return NO_DEFAULT;
    }

    snd_mixer_selem_register(handle, NULL, NULL);
    snd_mixer_load(handle);

    snd_mixer_selem_id_alloca(&sid);
    snd_mixer_selem_id_set_index(sid, 0);
    snd_mixer_selem_id_set_name(sid, SELEM_NAME);
    snd_mixer_elem_t *elem = snd_mixer_find_selem(handle, sid);

    if (!elem)
    {
        snd_mixer_close(handle);
        return NO_MASTER;
    }

    snd_mixer_selem_get_playback_volume_range(elem, &min, &max);
    snd_mixer_selem_set_playback_volume_all(elem, volume * max / 100);

    snd_mixer_close(handle);
    return OK;
}

int mute()
{
    snd_mixer_t *handle;
    snd_mixer_selem_id_t *sid;

    snd_mixer_open(&handle, 0);

    if (snd_mixer_attach(handle, CARD) != 0)
    {
        snd_mixer_close(handle);
        return NO_DEFAULT;
    }

    snd_mixer_selem_register(handle, NULL, NULL);
    snd_mixer_load(handle);

    snd_mixer_selem_id_alloca(&sid);
    snd_mixer_selem_id_set_index(sid, 0);
    snd_mixer_selem_id_set_name(sid, SELEM_NAME);
    snd_mixer_elem_t *elem = snd_mixer_find_selem(handle, sid);

    if (!elem)
    {
        snd_mixer_close(handle);
        return NO_MASTER;
    }

    if (snd_mixer_selem_has_playback_switch(elem))
    {
        snd_mixer_selem_set_playback_switch_all(elem, 0);
    }
    else
    {
        snd_mixer_close(handle);
        return NO_SWITCH;
    };

    snd_mixer_close(handle);
    return OK;
}

int unmute()
{
    snd_mixer_t *handle;
    snd_mixer_selem_id_t *sid;

    snd_mixer_open(&handle, 0);

    if (snd_mixer_attach(handle, CARD) != 0)
    {
        snd_mixer_close(handle);
        return NO_DEFAULT;
    }

    snd_mixer_selem_register(handle, NULL, NULL);
    snd_mixer_load(handle);

    snd_mixer_selem_id_alloca(&sid);
    snd_mixer_selem_id_set_index(sid, 0);
    snd_mixer_selem_id_set_name(sid, SELEM_NAME);
    snd_mixer_elem_t *elem = snd_mixer_find_selem(handle, sid);

    if (!elem)
    {
        snd_mixer_close(handle);
        return NO_MASTER;
    }

    if (snd_mixer_selem_has_playback_switch(elem))
    {
        snd_mixer_selem_set_playback_switch_all(elem, 1);
    }
    else
    {
        snd_mixer_close(handle);
        return NO_SWITCH;
    };

    snd_mixer_close(handle);
    return OK;
}

int is_muted(int *out)
{
    snd_mixer_t *handle;
    snd_mixer_selem_id_t *sid;

    snd_mixer_open(&handle, 0);

    if (snd_mixer_attach(handle, CARD) != 0)
    {
        snd_mixer_close(handle);
        return NO_DEFAULT;
    }

    snd_mixer_selem_register(handle, NULL, NULL);
    snd_mixer_load(handle);

    snd_mixer_selem_id_alloca(&sid);
    snd_mixer_selem_id_set_index(sid, 0);
    snd_mixer_selem_id_set_name(sid, SELEM_NAME);
    snd_mixer_elem_t *elem = snd_mixer_find_selem(handle, sid);

    if (!elem)
    {
        snd_mixer_close(handle);
        return NO_MASTER;
    }

    if (snd_mixer_selem_has_playback_switch(elem))
    {
        snd_mixer_selem_get_playback_switch(elem, 0, out);
    }
    else
    {
        snd_mixer_close(handle);
        return NO_SWITCH;
    };

    snd_mixer_close(handle);
    return OK;
}
